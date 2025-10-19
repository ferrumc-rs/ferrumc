use proc_macro::TokenStream;
use quote::quote;
use simd_json::prelude::{ValueAsObject, ValueAsScalar, ValueObjectAccess};
use simd_json::{OwnedValue, StaticNode};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Expr, Ident, Lit, LitStr, Result, Token};

const JSON_FILE: &[u8] = include_bytes!("../../../../../assets/data/blockstates.json");

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

    let name_str = if name.value().starts_with("minecraft:") {
        name.value()
    } else {
        format!("minecraft:{}", name.value())
    };
    let mut buf = JSON_FILE.to_vec();
    let v = simd_json::to_owned_value(&mut buf).unwrap();

    let filtered_names = v
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, v)| v.get("name").as_str() == Some(&name_str))
        .map(|(k, v)| (k.parse::<u32>().unwrap(), v))
        .collect::<Vec<_>>();

    let Some(opts) = opts else {
        if filtered_names.is_empty() {
            return syn::Error::new_spanned(
                name.clone(),
                format!("block '{}' not found in blockstates.json", name_str),
            )
            .to_compile_error()
            .into();
        }
        if filtered_names.len() > 1 {
            let properties = get_properties(&filtered_names);
            return syn::Error::new_spanned(
                name_str.clone(),
                format!(
                    "block '{}' has multiple variants, please specify properties. Available properties: {}",
                    name_str, pretty_print_props(&properties)
                ),
            )
            .to_compile_error()
            .into();
        }
        let first = filtered_names.iter().next().unwrap().0;
        return quote! { BlockId(#first) }.into();
    };

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
        .collect::<Result<std::collections::HashMap<String, String>>>();

    let props = match props {
        Ok(props) => props,
        Err(err) => return err.to_compile_error().into(),
    };

    let matched = filtered_names
        .iter()
        .filter(|(_, v)| {
            if let Some(map) = v.get("properties").as_object() {
                // eq impl from halfbwown
                if map.len() != props.len() {
                    return false;
                }
                return map.iter().all(|(key, value)| {
                    let converted = match value {
                        OwnedValue::Static(StaticNode::Bool(v)) => v.to_string(),
                        OwnedValue::Static(StaticNode::I64(v)) => v.to_string(),
                        OwnedValue::String(v) => v.to_string(),
                        _ => return false,
                    };
                    props.get(key).is_some_and(|v| *converted == *v)
                });
            }
            false
        })
        .map(|(id, _)| *id)
        .collect::<Vec<u32>>();

    if matched.is_empty() {
        let properties = get_properties(&filtered_names);
        if properties.is_empty() {
            return syn::Error::new_spanned(
                name_str.clone(),
                format!(
                    "block '{}' has no properties but the following properties were given: {}",
                    name_str.clone(),
                    pretty_print_given_props(opts)
                ),
            )
            .to_compile_error()
            .into();
        } else {
            return syn::Error::new_spanned(
                    name_str.clone(),
                    format!(
                        "no variant of block '{}' matches the specified properties. Available properties: {}",
                        name_str.clone(), pretty_print_props(&properties)
                    ),
                )
                    .to_compile_error()
                    .into();
        }
    }
    if matched.len() > 1 {
        return syn::Error::new_spanned(
                name_str.clone(),
                format!("block '{}' with specified properties has multiple variants, please refine properties", name_str),
            )
                .to_compile_error()
                .into();
    }

    let res = matched[0];
    quote! { BlockId(#res) }.into()
}

fn get_properties(filtered_names: &[(u32, &OwnedValue)]) -> Vec<(String, String)> {
    filtered_names
        .iter()
        .filter_map(|(_, v)| v.get("properties").and_then(|v| v.as_object()))
        .flat_map(|v| {
            v.iter().filter_map(|(k, v)| {
                let converted = match v {
                    OwnedValue::Static(StaticNode::Bool(v)) => v.to_string(),
                    OwnedValue::Static(StaticNode::I64(v)) => v.to_string(),
                    OwnedValue::String(v) => v.to_string(),
                    _ => return None,
                };
                Some((k.clone(), converted))
            })
        })
        .collect()
}

fn pretty_print_props(props: &[(String, String)]) -> String {
    let mut s = String::new();
    for (k, v) in props {
        s.push_str(&format!("{}: {}, ", k, v));
    }
    s.trim_end_matches(", ").to_string()
}

fn pretty_print_given_props(props: Opts) -> String {
    let mut s = String::new();
    for kv in props.pairs.iter() {
        let key = kv.key.to_string();
        let value = match &kv.value {
            Expr::Lit(v) => match &v.lit {
                Lit::Str(v) => v.value(),
                Lit::Bool(v) => v.value.to_string(),
                Lit::Int(v) => v.base10_digits().to_string(),
                _ => "unsupported".to_string(),
            },
            _ => "unsupported".to_string(),
        };
        s.push_str(&format!("{}: {}, ", key, value));
    }
    s.trim_end_matches(", ").to_string()
}
