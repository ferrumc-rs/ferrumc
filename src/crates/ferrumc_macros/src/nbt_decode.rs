use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{DeriveInput, Meta, parse_macro_input};

/*pub fn decode(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store all field decoding statements
    let mut statements = Vec::new();

    let is_nbtcompound = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("nbtcompound"));

    // Check if our struct has named fields
    if let syn::Data::Struct(syn::DataStruct {
                                 fields: syn::Fields::Named(fields),
                                 ..
                             }) = input.data
    {
        for field in fields.named {
            // Get the identifier of the field
            let ident = field.clone().ident.unwrap();
            // Generate a statement to decode this field from the bytes
            let type_name = field.clone().ty;
            let is_field_nbtcompound = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("nbtcompound"));
            let is_renamed = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("rename"));
            let name = if is_renamed {
                let rename = field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("rename"))
                    .unwrap();
                let meta = rename.clone().meta;
                match meta {
                    Meta::NameValue(meta) => {
                        meta.value.into_token_stream().to_string().replace("\"", "")
                    }
                    _ => panic!("Invalid rename attribute"),
                }
            } else {
                ident.to_string()
            };

            let is_optional = if let syn::Type::Path(type_path) = &type_name {
                type_path.path.segments.iter().any(|segment| {
                    segment.ident == "Option"
                })
            } else {
                false
            };
            let fail_message = format!("Tried to read {} that doesn't exist", name);
            if is_field_nbtcompound {
                if is_optional {
                    statements.push(
                        quote! {
                            #ident: match nbt.compound(#name) {
                                Some(compound) => <#type_name as crate::utils::impls::nbt_impls::NBTDecodable>::decode_from_compound(&compound, "").expect(#fail_message),
                                None => None,
                            },
                        }
                    );
                }else {
                    statements.push(
                        quote! {
                            #ident: <#type_name as crate::utils::impls::nbt_impls::NBTDecodable>::decode_from_compound(&nbt.compound(#name).expect(#fail_message), "").expect(#fail_message),
                        }
                    );
                }
            } else if is_nbtcompound {
                statements.push(
                    quote! {
                    #ident: <#type_name as crate::utils::impls::nbt_impls::NBTDecodable>::decode_from_compound(nbt, #name).expect(#fail_message),
                    },
                );
            } else {
                statements.push(
                    quote! {
                    #ident: <#type_name as crate::utils::impls::nbt_impls::NBTDecodable>::decode_from_base(&nbt, #name).expect(#fail_message),
                    },
                );
            };
        }
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = if is_nbtcompound {
        quote! {
            impl crate::utils::impls::nbt_impls::NBTDecodable for #name {

                fn decode_from_base(_: &simdnbt::borrow::BaseNbt, _: &str) -> core::result::Result<Self, crate::utils::error::Error>
                {
                    panic!("This should never be called");
                }

                fn decode_from_list(_: &simdnbt::borrow::NbtList) -> core::result::Result<Vec<Self>, crate::utils::error::Error>
                {
                    panic!("This should never be called");
                }

                fn decode_from_compound(nbt: &simdnbt::borrow::NbtCompound, _: &str) -> core::result::Result<Self, crate::utils::error::Error>
                {
                    Ok(Self {
                        #(#statements)*
                    })
                }
            }
        }
    } else {
        quote! {
        impl #name {
            pub fn decode(bytes: Vec<u8>) -> core::result::Result<Self, crate::utils::error::Error>
            {
                let nbt = simdnbt::borrow::read(&mut std::io::Cursor::new(bytes.as_slice())).unwrap().unwrap();
                Ok(Self {
                    #(#statements)*
                })

            }
        } }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
*/