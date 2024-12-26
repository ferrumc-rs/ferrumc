use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, Ident, ItemFn, LitBool, LitStr,
    Result as SynResult, Token,
};

static PENDING_ARGS: OnceLock<Mutex<HashMap<String, Vec<ArgAttr>>>> = OnceLock::new();

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
    name: String,
    parser: String,
    required: bool,
}

impl Parse for ArgAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name = input.parse::<LitStr>()?.value();
        input.parse::<Token![,]>()?;
        let parser = input.parse::<Ident>()?.to_string();

        let required = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse::<LitBool>()?.value()
        } else {
            true
        };

        Ok(ArgAttr {
            name,
            parser,
            required,
        })
    }
}

fn get_args_storage() -> &'static Mutex<HashMap<String, Vec<ArgAttr>>> {
    PENDING_ARGS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn arg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let arg_attr = parse_macro_input!(attr as ArgAttr);
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.sig.ident.to_string();

    let storage = get_args_storage();
    let mut pending_args = storage.lock().unwrap();
    pending_args
        .entry(fn_name)
        .or_default()
        .push(arg_attr);

    TokenStream::from(quote!(#input_fn))
}

pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    let command_attr = parse_macro_input!(attr as CommandAttr);
    let command_name = command_attr.name;

    let storage = get_args_storage();
    let mut pending_args = storage.lock().unwrap();
    let args = pending_args.remove(&fn_name_str).unwrap_or_default();

    let arg_names = args.iter().map(|arg| &arg.name).collect::<Vec<&String>>();
    let arg_parsers = args
        .iter()
        .map(|arg| format_ident!("{}", arg.parser))
        .collect::<Vec<Ident>>();
    let arg_required = args.iter().map(|arg| arg.required).collect::<Vec<bool>>();

    let register_fn_name = format_ident!("__register__{}_command", command_name);

    let expanded = quote! {
        #[ctor::ctor]
        fn #register_fn_name() {
            let command = Command {
                name: #command_name,
                args: vec![
                    #(
                        CommandArgument {
                            name: #arg_names.to_string(),
                            required: #arg_required,
                            parser: Box::new(#arg_parsers),
                        },
                    )*
                ],
                executor: executor(#fn_name)
            };

            let command = std::sync::Arc::new(command);
            register_command(command);
        }

        #input_fn
    };

    TokenStream::from(expanded)
}
