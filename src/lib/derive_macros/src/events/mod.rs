use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Expr, Lit, Meta, PatType};

pub fn event_handler_fn(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let input = parse_macro_input!(input as syn::ItemFn);

    let fn_name = &input.sig.ident;

    let priority = parse_priority(args);

    let register_fn_name = format_ident!("__register_listener_{}", fn_name);

    let (event_type, state) = extract_event_type(&input);
    let (event_type, state) = (event_type.ty, state.ty);

    let output = quote! {
        #input

        #[ctor::ctor]
        fn #register_fn_name() {
            // ::ferrumc_events::infrastructure::insert_into_events(
            // #event_type ::register(
            <#event_type as ::ferrumc_events::infrastructure::Event>::register(
                |ev: #event_type, state: #state| std::boxed::Box::pin(#fn_name(ev, state)),
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
        let Meta::NameValue(nv) = arg else {
            panic!("Expected a name-value attribute for the event handler");
        };

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

    event_priority
}

fn extract_event_type(input: &syn::ItemFn) -> (PatType, PatType) {
    let inputs = &input.sig.inputs;

    if inputs.len() != 2 {
        panic!("Expected the event handler to have exactly 2 arguments (the event, state)");
    }

    let syn::FnArg::Typed(pat_type) = &inputs[0] else {
        panic!("Expected the first argument to be a typed pattern");
    };
    let syn::FnArg::Typed(state) = &inputs[1] else {
        panic!("Expected the second argument to be a typed pattern");
    };

    // let syn::Path { segments, .. } = &type_path.path;

    (pat_type.clone(), state.clone())
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

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;

    // Check if I'm local or external (ferrumc_net)
    let found_crate = crate_name("ferrumc-net").unwrap();

    let net_crate = match found_crate {
        FoundCrate::Itself => {
            quote! {crate}
        }
        FoundCrate::Name(name) => {
            quote! {::#name}
        }
    };

    let output = quote! {
        impl ::ferrumc_events::infrastructure::Event for #name {
            type Data = Self;
            type State = #net_crate::GlobalState;
            type Error = #net_crate::errors::NetError;

            fn name() -> &'static str {
                stringify!(#name)
            }
        }
    };

    output.into()
}