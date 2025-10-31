use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Expr, Ident, Lit, LitStr, Result, Token};

pub(crate) mod matches;

// const JSON_FILE: &[u8] = include_bytes!("../../../../../assets/data/blockstates.json");

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

struct Input {
    name: LitStr,
    opts: Option<Opts>,
}

struct Opts {
    pairs: Punctuated<Kv, Token![,]>,
}

struct Kv {
    key: Ident,
    _colon: Token![:],
    value: Expr, // accept bools, strings, ints, calls, etc.
}

impl Parse for Kv {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _colon: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for Opts {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        Ok(Self {
            pairs: content.parse_terminated(Kv::parse, Token![,])?,
        })
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: LitStr = input.parse()?;
        let opts = if input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;
            Some(input.parse::<Opts>()?)
        } else {
            None
        };
        if !input.is_empty() {
            return Err(input.error("unexpected tokens after optional { ... }"));
        }
        Ok(Self { name, opts })
    }
}

pub fn block(input: TokenStream) -> TokenStream {
    let Input { name, opts } = syn::parse_macro_input!(input as Input);
    let name_value = name.value();
    let name_str = if let Some((_, name)) = name_value.split_once("minecraft:") {
        name
    } else {
        &name_value
    };

    let states = BLOCK_STATES.get(name_str);
    if states.is_none_or(|x| x.is_empty()) {
        return syn::Error::new_spanned(
            name.clone(),
            format!("block '{}' not found in blockstates.json", name_value),
        )
        .to_compile_error()
        .into();
    }

    let &states = states.unwrap();

    if opts.is_none() {
        if states.len() > 1 {
            return syn::Error::new_spanned(
                name,
                format!(
                    "block '{}' has multiple variants, please specify properties.",
                    name_value
                ),
            )
            .to_compile_error()
            .into();
        }
        let id = states[0];
        return quote! { BlockId(#id) }.into();
    }

    let opts = opts.unwrap();

    let props = opts
        .pairs
        .iter()
        .map(|kv| {
            Ok((
                kv.key.to_string(),
                match &kv.value {
                    Expr::Lit(v) => match &v.lit {
                        Lit::Str(v) => v.value(),
                        Lit::Bool(v) => v.value.to_string(),
                        Lit::Int(v) => v.base10_digits().to_string(),
                        _ => return Err(syn::Error::new_spanned(
                            &kv.value,
                            "only string, bool, and int literals are supported as property values",
                        )),
                    },
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &kv.value,
                            "only string, bool, and int literals are supported as property values",
                        ))
                    }
                },
            ))
        })
        .collect::<Result<Vec<(String, String)>>>();

    let props = match props {
        Ok(props) => props,
        Err(err) => return err.to_compile_error().into(),
    };

    fn intersect(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut v = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < a.len() && j < b.len() {
            match a[i].cmp(&b[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => {
                    v.push(a[i]);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Greater => j += 1,
            }
        }
        v
    }

    let mut states = states.to_owned();

    for (k, v) in props {
        let key = format!("{}:{}", k, v);
        let prop_states = BLOCK_STATES.get(&key);
        if prop_states.is_none_or(|x| x.is_empty()) {
            return syn::Error::new_spanned(
                name.clone(),
                format!("No block has properties '{}'.", key),
            )
            .to_compile_error()
            .into();
        }

        let &prop_states = prop_states.unwrap();
        states = intersect(&states, prop_states);
        if states.is_empty() {
            return syn::Error::new_spanned(
                name,
                format!(
                    "No variant of block '{}' matches the specified properties.",
                    name_value
                ),
            )
            .to_compile_error()
            .into();
        }
    }

    if states.len() > 1 {
        return syn::Error::new_spanned(
                name,
                format!("Block '{}' with specified properties has multiple variants ({:?}), please refine properties", name_value, states),
            )
                .to_compile_error()
                .into();
    }

    let id = states[0];

    return quote! { BlockId(#id) }.into();
}
