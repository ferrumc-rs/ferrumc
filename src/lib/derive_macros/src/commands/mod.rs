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

    let mut parser_fields = Vec::new();
    let mut result_fields = Vec::new();
    let mut field_conversions = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let is_sender = field.attrs.iter().any(|attr| {
            let Meta::Path(path) = &attr.meta else {
                return false;
            };

            path.is_ident("sender")
        });

        if is_sender {
            result_fields.push(quote! {
                #field_name: #field_type
            });
            field_conversions.push(quote! {
                #field_name: "" // TODO
            });
        } else {
            parser_fields.push(quote! {
                #field_name: Box<dyn ::ferrumc_commands::arg::ArgumentParser>
            });
            result_fields.push(quote! {
                #field_name: #field_type
            });
            field_conversions.push(quote! {
                #field_name: ctx.arg(stringify!(#field_name))
            });
        }
    }

    let expanded = quote! {
        struct #parser_struct_name {
            #(#parser_fields,)*
        }

        struct #struct_name {
            #(#result_fields,)*
        }

        impl #struct_name {
            async fn execute(command: #struct_name) -> ::ferrumc_commands::CommandResult {
                Ok(TextComponentBuilder::new("").build())
            }

            #[ctor::ctor]
            fn register() {
                let command = Arc::new(Command {
                    name: #(#command_names,)*[0],
                    args: vec![
                        #(::ferrumc_commands::arg::CommandArgument {
                            name: stringify!(#parser_fields).to_string(),
                            required: true,
                            parser: self.#parser_fields.clone(),
                        },)*
                    ],
                    executor: executor(|ctx: Arc<::ferrumc_commands::ctx::CommandContext>| async move {
                        let command = #struct_name {
                            #(#field_conversions,)*
                        };

                        #struct_name::execute(command)
                    }),
                });

                for &name in &[#(#command_names,)*] {
                    ::ferrumc_commands::infrastructure::register_command(Arc::clone(&command));
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Gets the command names, i.e. primary name and aliases from an attribute list
fn get_command_names(attrs: &[Attribute]) -> Vec<String> {
    for attr in attrs {
        if let Meta::List(_) = &attr.meta {
            if attr.path().is_ident("command") {
                let mut names = Vec::new();

                if let Err(_) = attr.parse_nested_meta(|meta| {
                    if let Some(ident) = meta.path.get_ident() {
                        names.push(ident.to_string());
                    }
                    Ok(())
                }) {
                    continue;
                }

                return names;
            }
        }
    }
    vec![]
}
