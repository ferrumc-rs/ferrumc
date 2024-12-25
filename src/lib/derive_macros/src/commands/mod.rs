use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, DeriveInput, Meta};

pub fn derive_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let command_names = get_command_names(&input.attrs);

    let struct_name = &input.ident;
    let parser_struct_name = format_ident!("__{}Parser", struct_name);

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Command can only be derived for structs"),
    };

    let mut parser_field_names = Vec::new();
    let mut parser_field_types = Vec::new();
    let mut parser_fields = Vec::new();
    let mut result_fields = Vec::new();
    let mut field_conversions = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let is_sender = field.attrs.iter().any(|attr| {
            if let Meta::Path(path) = &attr.meta {
                return path.is_ident("sender");
            }
            false
        });

        if is_sender {
            result_fields.push(quote! {
                #field_name: #field_type
            });
            field_conversions.push(quote! {
                #field_name: "rad" // TODO: Replace with actual sender retrieval logic
            });
        } else {
            parser_fields.push(quote! {
                #field_name: Box::new(<#field_type as crate::arg::parser::ArgumentParser>::Output::default())
            });
            parser_field_names.push(field_name.clone());
            parser_field_types.push(quote! {
                #field_name: Box<dyn crate::arg::ArgumentParser>
            });
            result_fields.push(quote! {
                #field_name: <#field_type as crate::arg::ArgumentParser>::Output
            });
            field_conversions.push(quote! {
                #field_name: ctx.arg(stringify!(#field_name))
            });
        }
    }

    let command_name = command_names.first().expect("command has no name");

    let expanded = quote! {
        struct #parser_struct_name {
            #(#parser_field_types,)*
        }

        struct #struct_name {
            #(#result_fields,)*
        }

        impl #struct_name {
            async fn execute(command: #struct_name) -> crate::CommandResult {
                Ok(ferrumc_text::builders::TextComponentBuilder::new("").build())
            }

            fn register() {
                static INIT: std::sync::Once = std::sync::Once::new();
                INIT.call_once(|| {
                    let command = std::sync::Arc::new(crate::Command {
                        name: #command_name,
                        args: vec![
                            #(crate::arg::CommandArgument {
                                name: stringify!(#parser_field_names).to_string(),
                                required: true,
                                parser: Box::new(#parser_fields),
                            },)*
                        ],
                        executor: crate::executor(|ctx: std::sync::Arc<crate::ctx::CommandContext>| async move {
                            let command = #struct_name {
                                #(#field_conversions,)*
                            };

                            #struct_name::execute(command).await
                        }),
                    });

                    for &name in &[#(#command_names,)*] {
                        crate::infrastructure::register_command(std::sync::Arc::clone(&command));
                    }
                });
            }
        }
    };
    
    println!("{expanded}");

    TokenStream::from(expanded)
}

fn get_command_names(attrs: &[Attribute]) -> Vec<String> {
    for attr in attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("command") {
                let mut names = Vec::new();
                let input = meta_list.clone().tokens.to_string();
                for name in input.split(", ") {
                    names.push(name.to_string());
                }
                return names;
            }
        }
    }
    vec![]
}
