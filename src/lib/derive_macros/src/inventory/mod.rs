use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Data, Visibility};

pub fn create(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    if let Data::Struct(data_struct) = &input.data {
        let mut default_statements = Vec::new();
        let mut field_statements = Vec::new();

        let found_crate = crate_name("ferrumc-inventory").unwrap();

        let net_crate = match found_crate {
            FoundCrate::Itself => {
                quote! { crate }
            }
            FoundCrate::Name(name) => {
                let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
                quote! { #name }
            }
        };

        let mut inventory_type_creator = quote! {};

        // Extract inventory_type attribute
        for attr in &input.attrs {
            if attr.path().is_ident("inventory") {
                attr.parse_nested_meta(|meta| {
                    if let Some(ident) = meta.path.get_ident() {
                        let ident_str = ident.to_string();
                        let ident_str_ident = syn::Ident::new(&ident_str, proc_macro2::Span::call_site());
                        match ident_str.as_str() {
                            "inventory_type" => {
                                if let Some(ident) = handle_meta("inventory_type", &meta, true) {
                                    inventory_type_creator = quote! {
                                        #inventory_type_creator
                                        .#ident_str_ident(#net_crate::inventory::InventoryType::#ident)
                                    };
                                }
                            },
                            "is_synced" => {
                                if let Some(expr) = handle_meta("is_synced", &meta, false) {
                                    inventory_type_creator = quote! {
                                        #inventory_type_creator
                                        .#ident_str_ident(#expr)
                                    };
                                }
                            },
                            "title" => {
                                if let Some(expr) = handle_meta("title", &meta, false) {
                                    inventory_type_creator = quote! {
                                        #inventory_type_creator
                                        .#ident_str_ident(ferrumc_text::TextComponent::new(#expr).build())
                                    };
                                }
                            }
                            _ => {},
                        }
                    }
                    Ok(())
                })
                    .unwrap();
            }
        }

        // Process fields
        for field in &data_struct.fields {
            let field_name = &field.ident.clone().expect("Missing field");
            let field_ty = &field.ty;

            let mut id_expr = None;
            let mut value_expr = None;

            if let Visibility::Public(_) = &field.vis {
                for attr in &field.attrs {
                    if attr.path().is_ident("slot") {
                        attr.parse_nested_meta(|meta| {
                            if let Some(ident) = meta.path.get_ident() {
                                let ident_str = ident.to_string();
                                match ident_str.as_str() {
                                    "id" => {
                                        id_expr = handle_meta("id", &meta, true);
                                    }
                                    "default_value" => {
                                        value_expr = handle_meta("default_value", &meta, true);
                                    }
                                    _ => {}
                                }
                            }
                            Ok(())
                        })
                        .unwrap();
                    }
                }
            }

            // Generate default initialization and setter methods
            if let (Some(id), Some(value)) = (id_expr, value_expr) {
                default_statements.push(quote! {
                    #field_name: #value,
                });

                let setter_name =
                    syn::Ident::new(&format!("set_{}", field_name), field_name.span());
                field_statements.push(quote! {
                    pub fn #setter_name<S: Into<Slot> + Copy>(&mut self, #field_name: S) {
                        self.#field_name = #field_name.into();
                        self.set_slot(#id, #field_name);
                    }
                });
            }
        }

        // Generate the `new` method
        let new_method = quote! {
            pub fn new(id: u8) -> Self {
                Self {
                    inventory: #net_crate::builder::InventoryBuilder::new(id)
                        #inventory_type_creator
                        .build(),
                    #(#default_statements)*
                }
            }
        };

        // Generate the complete implementation block
        // Wacky ass code because rust is retarded
        let output = quote! {
            impl #name {
                #new_method

                #(#field_statements)*
            }

            impl std::ops::Deref for #name {
                type Target = Inventory;

                fn deref(&self) -> &Self::Target {
                    &self.inventory
                }
            }

            impl std::ops::DerefMut for #name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.inventory
                }
            }
        };

        output.into()
    } else {
        TokenStream::new()
    }
}

fn handle_meta(
    name: &str,
    meta: &syn::meta::ParseNestedMeta,
    is_required: bool,
) -> Option<syn::Expr> {
    match meta.value() {
        Ok(value) => {
            if let Ok(value) = value.parse::<syn::Expr>() {
                Some(value)
            } else if is_required {
                panic!("Failed to parse value for attribute '{}'", name);
            } else {
                None
            }
        }
        Err(_) => {
            if is_required {
                panic!("Missing required attribute '{}'", name);
            }

            None
        }
    }
}

pub fn inventory_type(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
