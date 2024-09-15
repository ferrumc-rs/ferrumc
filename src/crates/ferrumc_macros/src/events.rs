use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta, Type};
use syn::punctuated::Punctuated;

pub(super) fn derive_event_handler(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    const EVENT_HANDLER_KEY: &str = "event_handler";
    const PRIORITY_KEY: &str = "priority";
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


    input.attrs.iter().for_each(|attr| {
        if !attr.path().is_ident(EVENT_HANDLER_KEY) {
            println!("Skipping attribute: {}", attr.path().get_ident().unwrap().to_string());
            return;
        }

        attr.parse_nested_meta(|meta| {
            if !meta.path.is_ident(PRIORITY_KEY) {
                panic!("Expected a priority attribute for the event handler");
            }

            let value = meta.value().expect("Expected a value for the priority attribute");

            if let Ok(string) = value.parse::<syn::LitStr>() {
                let value = string.value();
                event_priority = get_priority(&value);
            }else if let Ok(int) = value.parse::<syn::LitInt>() {
                event_priority = int.base10_parse::<u8>().expect("Expected a number for the priority attribute");
            }else {
                panic!("Expected a string literal for the priority attribute. Possible values are: fastest, fast, normal, slow, slowest, or values between 0 and 255. Where 0 is the fastest and 255 is the slowest");
            }

            Ok(())
        }).expect("Failed to parse priority attribute");
    });

    let expanded = quote! {
        impl #name {
            pub const fn new() -> Self {
                Self
            }
        }

        inventory::submit! {
            crate::events::registry::EventContainer::new(
                #event_priority,
                & #name::new(),
            )
        }
    };

    TokenStream::from(expanded)
}

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

    println!("Priority: {}", event_priority);


    // Parse the function
    let input_fn = parse_macro_input!(input as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;

    let (event_type, is_mutable) = match inputs.first().expect("Expected event type as the argument. e.g. event: &mut MoveEvent") {
        syn::FnArg::Typed(typed) => {
            // let ty = &typed.ty;
            let Type::Reference(type_ref) = &*typed.ty else {
                panic!("Input must be a reference to the event type. e.g. event: &mut MoveEvent")
            };
            let is_mutable = type_ref.mutability.is_some();
            let ty = &type_ref.elem;
            (ty, is_mutable)
        }
        _ => panic!("Expected a typed argument for the event handler")
    };

    let handler_path = if is_mutable {
        quote! { crate::events::registry::FunctionEventHandlerMut::<#event_type> }
    } else {
        quote! { crate::events::registry::FunctionEventHandlerRef::<#event_type> }
    };

    let expanded = quote! {
        #input_fn

        inventory::submit! {
            crate::events::registry::EventContainer::new(
                #event_priority,
                & #handler_path {
                    handler: #fn_name,
                }
            )
        }
    };

    TokenStream::from(expanded)
}