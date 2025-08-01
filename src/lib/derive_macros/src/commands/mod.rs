use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, FnArg, ItemFn, LitStr, Pat, Result as SynResult, Type,
};

#[derive(Clone, Debug)]
struct Arg {
    parser: String,
    name: String,
    required: bool,
    ty: String,
}

struct CommandAttr {
    name: String,
}

impl Parse for CommandAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name = input.parse::<LitStr>()?.value();
        Ok(CommandAttr { name })
    }
}

struct ArgAttr {
    parser: String,
}

impl Parse for ArgAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let parser = input.parse::<Expr>()?.to_token_stream().to_string();

        Ok(ArgAttr { parser })
    }
}

pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.clone().sig.ident;

    let command_attr = parse_macro_input!(attr as CommandAttr);

    let mut args = Vec::new();
    let mut bevy_args = Vec::<(Box<Pat>, Type)>::new();
    let mut has_sender_arg = false;

    for fn_arg in &mut input_fn.sig.inputs {
        let FnArg::Typed(fn_arg) = fn_arg else {
            return TokenStream::from(quote! {
                compiler_error!("command handler cannot have receiver");
            });
        };

        if fn_arg.attrs.is_empty() {
            bevy_args.push((fn_arg.pat.clone(), *fn_arg.ty.clone()));
        }

        let mut sender_arg_mismatched_ty = false;

        fn_arg.attrs.retain(|arg| {
            let is_parser = arg.path().is_ident("parser");
            let is_sender = arg.path().is_ident("sender");

            if is_parser {
                let required = match *fn_arg.ty {
                    Type::Path(ref path) => {
                        path.path.segments.iter().any(|seg| seg.ident == "Option")
                    }
                    _ => false,
                };

                args.push(Arg {
                    parser: arg.parse_args_with(ArgAttr::parse).unwrap().parser,
                    name: fn_arg.pat.to_token_stream().to_string(),
                    required,
                    ty: fn_arg.ty.to_token_stream().to_string(),
                });
            }

            if is_sender {
                match *fn_arg.ty {
                    Type::Path(ref path) => {
                        if !path
                            .path
                            .segments
                            .iter()
                            .any(|seg| seg.ident.to_string() == "Entity")
                        {
                            sender_arg_mismatched_ty = true;
                            return false;
                        }
                    }

                    _ => {
                        sender_arg_mismatched_ty = true;
                        return false;
                    }
                }
                has_sender_arg = true;
            }

            !is_parser && !is_sender
        });

        if sender_arg_mismatched_ty {
            return TokenStream::from(quote! {
                compile_error!("invalid type for sender arg - should be Entity");
            });
        }
    }

    let command_struct_name = format_ident!("__command_{}", fn_name);
    let arg_fields = args
        .clone()
        .iter()
        .map(|arg| {
            let ty = syn::parse_str::<Type>(&arg.ty).expect("invalid arg type");
            let name = format_ident!("{}", arg.name);

            quote! { #name: #ty, }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let system_name = format_ident!("__{}_handler", command_attr.name.clone());
    let system_args = bevy_args
        .clone()
        .iter()
        .map(|arg| {
            let (pat, ty) = arg;
            quote! { #pat: #ty, }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let system_arg_pats = bevy_args
        .clone()
        .iter()
        .map(|arg| {
            let (pat, _) = arg;
            quote!(#pat)
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let arg_extractors = args
        .clone()
        .iter()
        .map(|arg| {
            let name = &arg.name;
            let ty = syn::parse_str::<Type>(&arg.ty).expect("invalid arg type");

            quote! { ctx.arg::<#ty>(#name), }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let sender_param = if has_sender_arg {
        quote! { sender.clone(), }
    } else {
        quote! {}
    };

    let ctor_fn_name = format_ident!("__{}_register", command_attr.name.clone());
    let command_name = command_attr.name;

    let command_args = args
        .iter()
        .map(|arg| {
            let parser = syn::parse_str::<Expr>(&arg.parser.clone()).expect("invalid parser");
            let name = arg.name.clone();
            let required = arg.required;
            
            quote! {
                ferrumc_commands::arg::CommandArgument {
                    name: #name.to_string(),
                    required: #required,
                    parser: Box::new(#parser),
                },
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    TokenStream::from(quote! {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        struct #command_struct_name {
            #(#arg_fields)*
            __sender: bevy_ecs::prelude::Entity,
            __input: ::ferrumc_commands::input::CommandInput,
        }

        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[doc(hidden)]
        #input_fn

        #[allow(non_snake_case)]
        #[allow(unused_variables)] // if there is no sender arg
        #[doc(hidden)]
        fn #system_name(mut events: bevy_ecs::prelude::EventReader<ferrumc_commands::events::ResolvedCommandDispatchEvent>, #(#system_args)*) {
            for ferrumc_commands::events::ResolvedCommandDispatchEvent { command, ctx, sender } in events.read() {
                if command.name == #command_name {
                    #fn_name(#(#arg_extractors)* #sender_param #(#system_arg_pats)*);
                }
            }
        }

        #[ctor::ctor]
        #[doc(hidden)]
        fn #ctor_fn_name() {
            ferrumc_commands::infrastructure::add_system(#system_name);

            ferrumc_commands::infrastructure::register_command(std::sync::Arc::new(ferrumc_commands::Command {
                name: #command_name,
                args: vec![#(#command_args)*],
            }));
        }
    })
}
