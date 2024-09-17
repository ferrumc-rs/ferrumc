use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Expr, Lit, Meta};
use syn::punctuated::Punctuated;
use syn::token::Comma;

pub fn event_handler_fn(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let input = parse_macro_input!(input as syn::ItemFn);

    let fn_name = &input.sig.ident;
    
    let priority = parse_priority(args);

    let register_fn_name = format_ident!("__register_listener_{}", fn_name);
    
    let event_type = extract_event_type(&input);

    let output = quote! {
        #input
        
        #[ctor::ctor]
        fn #register_fn_name() {
            ::ferrumc_events::infrastructure::insert_into_events(
                |ev: #event_type| std::boxed::Box::pin(#fn_name(ev)),
                #priority
            );
        }
    };
    
    output.into()
}

fn parse_priority(args: Punctuated<Meta, Comma>) -> u8 {
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

    event_priority
}

fn extract_event_type(input: &syn::ItemFn) -> syn::Type {
    let inputs = &input.sig.inputs;
    
    if inputs.len() != 1 {
        panic!("Expected the event handler to have exactly one argument (the event)");
    }
    
    let syn::FnArg::Typed(pat_type) = &inputs[0] else {
        panic!("Expected the first argument to be a typed pattern");
    };
    
    // let syn::Path { segments, .. } = &type_path.path;
    
    *pat_type.ty.clone()
}

// #[ctor::ctor]
// fn __register_some_event_listener() {
//     insert_into_events(|ev: Arc<RwLock<SomeEvent>>| Box::pin(some_event_listener(ev)), 0);
// }
//
// async fn some_event_listener(event: Arc<RwLock<SomeEvent>>) {
//     let mut ev = event.write();
//     ev.data = 10;
//     println!("I set the event's data to 10");
// }