use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};

pub(super) fn derive_event_handler(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;



    let expanded = quote! {
        impl #name {
            pub const fn new() -> Self {
                Self
            }
        }

        inventory::submit! {
            crate::events::registry::EventContainer {
                priority: crate::events::registry::EventPriority::default(),
                handler: & #name::new(),
            }
        }
    };

    TokenStream::from(expanded)
}