use alloc::vec::Vec;
use core::convert::{Into, TryFrom};
use core::iter::Iterator;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn enum_discriminant_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;

    let data_enum = match input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(
                enum_name,
                "EnumDiscriminant can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut arms = Vec::with_capacity(data_enum.variants.len());

    for (idx, variant) in data_enum.variants.iter().enumerate() {
        let v_ident = &variant.ident;
        let idx_i32 = i32::try_from(idx).unwrap_or(i32::MAX);

        let pat = match &variant.fields {
            Fields::Unit => quote! { Self::#v_ident },
            Fields::Unnamed(_) => quote! { Self::#v_ident ( .. ) },
            Fields::Named(_) => quote! { Self::#v_ident { .. } },
        };

        arms.push(quote! { #pat => #idx_i32, });
    }

    let expanded = quote! {
        impl #enum_name {
            #[inline]
            pub fn discriminant(&self) -> i32 {
                match self {
                    #( #arms )*
                }
            }
        }
    };

    expanded.into()
}
