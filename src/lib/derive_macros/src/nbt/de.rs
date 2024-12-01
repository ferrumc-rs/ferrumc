use crate::nbt::helpers::NbtFieldAttribute;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::helpers::StructInfo;

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let syn::Data::Struct(_) = input.data else {
        panic!("Only structs are supported!");
    };

    /*let struct_name = &input.ident;
    let mut impl_generics = input.generics.clone();
    let ty_generics = input.generics.split_for_impl().1;
    let where_clause = &input.generics.where_clause;

    // Introduce a new lifetime 'de for deserialization
    let has_lifetime = impl_generics.params.iter().any(|param| matches!(param, syn::GenericParam::Lifetime(_)));
    if !has_lifetime {
        let de_lifetime = Lifetime::new("'de", Span::call_site());
        impl_generics.params.insert(
            0,
            syn::GenericParam::Lifetime(syn::LifetimeParam::new(de_lifetime.clone())),
        );
    }

    let lifetime = impl_generics
        .params
        .iter()
        .find_map(|param| match param {
            syn::GenericParam::Lifetime(lifetime) => Some(lifetime.lifetime.clone()),
            _ => None,
        })
        .unwrap();

    let (impl_generics, _, _) = impl_generics.split_for_impl();*/
    let StructInfo {
        struct_name,
        impl_generics,
        ty_generics,
        where_clause,
        lifetime,
        lifetime_without_ident,
        force_created
    } = crate::helpers::extract_struct_info(&input, Some("'de"));

    let fields = crate::helpers::get_fields(&input);
    let fields_init = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        let mut deserialize_name = field_name.to_string();

        let attributes = NbtFieldAttribute::from_field(field);

        let mut optional = false;
        let mut skip = false;

        for attr in attributes {
            match attr {
                NbtFieldAttribute::Rename { new_name } => {
                    deserialize_name = new_name;
                }
                NbtFieldAttribute::Optional => {
                    optional = true;
                }
                NbtFieldAttribute::Skip => {
                    skip = true;
                }
                _ => {}
            }
        }

        let elem_name = format!("{} (field: {})", deserialize_name, field_name);

        if skip {
            return quote! {
                #field_name: Default::default(),
            };
        }

        if optional {
            return quote! {
                #field_name: element.get(#deserialize_name).map_or(Ok(None), |e| {
                    <#field_ty as ::ferrumc_nbt::FromNbt #lifetime>::from_nbt(tapes, e)
                })?,
            };
        }

        quote! {
            #field_name: <#field_ty as ::ferrumc_nbt::FromNbt #lifetime>::from_nbt(
                tapes,
                element.get(#deserialize_name).ok_or({
                    ::ferrumc_nbt::NBTError::ElementNotFound(#elem_name)
                })?
            )?,
        }
    });
    
    let impl_generics = if force_created {
        quote! { <'de> #impl_generics }
    } else {
        quote! { #impl_generics }
    };
    
    let expanded = quote! {
        impl #impl_generics ::ferrumc_nbt::FromNbt #lifetime for #struct_name #ty_generics #where_clause {
            fn from_nbt(
                tapes: &::ferrumc_nbt::NbtTape #lifetime,
                element: &::ferrumc_nbt::NbtTapeElement #lifetime
            ) -> ::ferrumc_nbt::Result<Self> {
                Ok(#struct_name {
                    #(#fields_init)*
                })
            }
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn from_bytes(bytes: &#lifetime_without_ident [u8]) -> ::ferrumc_nbt::Result<Self> {
                let mut tape = ::ferrumc_nbt::NbtTape::new(bytes);
                tape.parse();
                let root = tape.root.as_ref()
                    .map(|(_, b)| b)
                    .ok_or(::ferrumc_nbt::NBTError::NoRootTag)?;
                <#struct_name #ty_generics as ::ferrumc_nbt::FromNbt #lifetime>::from_nbt(&tape, root)
            }
        }
    };

    TokenStream::from(expanded)
}
