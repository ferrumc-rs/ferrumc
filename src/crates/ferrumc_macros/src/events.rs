use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{parse_macro_input, Expr, Lit, Meta};
use syn::punctuated::Punctuated;

pub(super) fn event_handler(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let args = parse_macro_input!(args with Punctuated::<Meta, syn::Token![,]>::parse_terminated);


    fn get_priority(priority: &str) -> u8 {
        match priority {
            "fastest" => 0,
            "fast" => 64,
            "normal" => 128,
            "slow" => 192,
            "slowest" => 255,
            _ => 128,
        }
    }

    let mut event_priority = 128;

    for arg in args.iter() {
        match arg {
            Meta::NameValue(nv) => {
                if !nv.path.is_ident("priority") {
                    continue;
                }
                let value = &nv.value;

                let Expr::Lit(value) = value else {
                    panic!("Expected a priority attribute for the event handler");
                };

                let value = &value.lit;

                match value {
                    Lit::Str(val) => {
                        let value = val.value();
                        event_priority = get_priority(&value);
                    }
                    Lit::Int(int) => {
                        event_priority = int.base10_parse::<u8>().expect("Expected a number for the priority attribute");
                    }
                    _ => panic!("Expected a string literal for the priority attribute. Possible values are: fastest, fast, normal, slow, slowest, or values between 0 and 255. Where 0 is the fastest and 255 is the slowest")
                };

            }
            _ => {}
        }
    }

    // Parse the function
    let input_fn = parse_macro_input!(input as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;

    let event_type = match inputs.first().expect("Expected event type as the argument. e.g. event: &mut MoveEvent") {
        syn::FnArg::Typed(typed) => {
            // &typed.ty
            let event_type = &typed.ty;
            // remove `Arc<` and `>` cuz i dont know how to do it in the event handling :skull:
            let event_type = event_type.to_token_stream().to_string();
            let regex = Regex::new(r#"Arc.<.(.*).>"#).unwrap();
            let event_type = regex.replace_all(&event_type, "$1").to_string();
            let event_type: syn::Type = syn::parse_str(&event_type).expect("Failed to parse the event type");
            event_type
        }
        _ => panic!("Expected a typed argument for the event handler")
    };

    let handler_path = quote! { crate::events::creation::registry::FunctionEventHandler::<#event_type> };

    let expanded = quote! {
        #input_fn

        inventory::submit! {
            crate::events::creation::registry::EventContainer::new(
                #event_priority,
                /*#handler_path {
                    handler: &#fn_name,
                }*/
                &#handler_path {
                    handler: |event, state| Box::pin(#fn_name(event, state)),
                }
            )
        }
    };

    TokenStream::from(expanded)
}