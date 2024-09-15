extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Lit, Meta, Type};

mod decode;
mod ecs;
mod encode;
mod nbt_decode;
mod packet;
mod utils;
mod events;

#[proc_macro_derive(NetDecode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    decode::derive(input)
}

#[proc_macro_derive(NetEncode, attributes(encode))]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    encode::derive(input)
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {
    packet::attribute(args, input)
}
/*
#[proc_macro_derive(NBTDecode, attributes(nbtcompound, rename))]
pub fn nbt_decode_derive(input: TokenStream) -> TokenStream {
    nbt_decode::decode(input)
}*/

#[proc_macro]
pub fn bake_packet_registry(input: TokenStream) -> TokenStream {
    packet::bake(input)
}

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    ecs::derive_component(input)
}
#[proc_macro_derive(Constructor)]
pub fn derive_constructor(input: TokenStream) -> TokenStream {
    ecs::derive_constructor(input)
}

#[proc_macro_derive(AutoGenName)]
pub fn derive_name(input: TokenStream) -> TokenStream {
    utils::derive_name(input)
}

#[proc_macro_derive(Getter)]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    utils::derive_getter(input)
}

#[proc_macro_derive(EventHandler, attributes(priority))]
pub fn derive_event_handler(input: TokenStream) -> TokenStream {
    events::derive_event_handler(input)
}

#[proc_macro_attribute]
pub fn event_handler(args: TokenStream, input: TokenStream) -> TokenStream {
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