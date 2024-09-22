use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lifetime};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = crate::helpers::get_fields(&input);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Determine the lifetime to use in FromNbt
    let nbt_lifetime = if let Some(lt) = input.generics.lifetimes().next() {
        lt.lifetime.clone()
    } else {
        Lifetime::new("'_", Span::call_site())
    };

    let fields_init = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        quote! {
            #field_name: <#field_ty as ::ferrumc_nbt::FromNbt<#nbt_lifetime>>::from_nbt(
                tapes,
                element.get(stringify!(#field_name)).ok_or_else(|| {
                    ::ferrumc_nbt::NBTError::ElementNotFound(stringify!(#field_name))
                })?
            )?,
        }
    });

    let expanded = quote! {
        impl #impl_generics ::ferrumc_nbt::FromNbt<#nbt_lifetime> for #struct_name #ty_generics #where_clause {
            fn from_nbt(
                tapes: &#nbt_lifetime ::ferrumc_nbt::NbtTape,
                element: &#nbt_lifetime ::ferrumc_nbt::NbtTapeElement
            ) -> ::ferrumc_nbt::Result<Self> {
                Ok(#struct_name {
                    #(#fields_init)*
                })
            }
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn from_bytes(bytes: &#nbt_lifetime[u8]) -> ::ferrumc_nbt::Result<Self> {
                let mut tape = ::ferrumc_nbt::NbtTape::new(bytes);
                tape.parse();
                let root = tape.root.as_ref()
                    .map(|(_, b)| b)
                    .ok_or(::ferrumc_nbt::NBTError::NoRootTag)?;
                <#struct_name #ty_generics as ::ferrumc_nbt::FromNbt<#nbt_lifetime>>::from_nbt(&tape, root)
            }
        }
    };

    TokenStream::from(expanded)
}
