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

        let mut inventory_type_expr: Option<syn::Expr> = None;

        // Extract inventory_type attribute
        for attr in &input.attrs {
            if attr.path().is_ident("inventory_type") {
                attr.parse_nested_meta(|meta| {
                    if let Some(ident) = meta.path.get_ident() {
                        if ident.to_string().as_str() == "value" {
                            let value = meta.value().expect("Missing value for inventory_type");

                            let value = value
                                .parse::<syn::Expr>()
                                .expect("Failed to parse value in inventory_type");
                            inventory_type_expr = Some(value);
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
                                match ident.to_string().as_str() {
                                    "id" => {
                                        let value = meta.value().expect("Missing value for slot");

                                        let value = value
                                            .parse::<syn::Expr>()
                                            .expect("Failed to parse value in slot");

                                        id_expr = Some(value);
                                    }
                                    "default_value" => {
                                        let value =
                                            meta.value().expect("Missing default_value for slot");

                                        let value = value
                                            .parse::<syn::Expr>()
                                            .expect("Failed to parse default_value in slot");

                                        value_expr = Some(value);
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
                    pub fn #setter_name(&mut self, #field_name: #field_ty) {
                        self.#field_name = #field_name;
                        self.set_slot(#id, #net_crate::slot::Slot::with_item(#field_name));
                    }
                });
            }
        }

        // Generate the `new` method
        let new_method = inventory_type_expr.map(|expr| {
            quote! {
                pub fn new(id: u8) -> Self {
                    Self {
                        inventory: #net_crate::builder::InventoryBuilder::new(id)
                            .inventory_type(#net_crate::inventory::InventoryType::#expr)
                            .build(),
                        #(#default_statements)*
                    }
                }
            }
        });

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

pub fn inventory_type(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub fn slot(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
