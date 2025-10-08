use proc_macro::{quote, TokenStream};
use simd_json::prelude::{ValueAsObject, ValueAsScalar, ValueObjectAccess};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Expr, Ident, LitStr, Result, Token};

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
        .filter(|v| {
            let name_in_json = v.1.get("name");
            if let Some(resolved_name) = name_in_json {
                resolved_name.as_str() == Some(&name_str)
            } else {
                false
            }
        })
        .map(|v| (v.0.parse::<u32>().unwrap(), v.1))
        .collect::<Vec<_>>();
    if let Some(opts) = opts {
        let mut props = vec![];
        for kv in opts.pairs.iter() {
            let key = kv.key.to_string();
            let value = match &kv.value {
                Expr::Lit(expr_lit) => {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        lit_str.value()
                    } else if let syn::Lit::Bool(lit_bool) = &expr_lit.lit {
                        lit_bool.value.to_string()
                    } else if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                        lit_int.base10_digits().to_string()
                    } else {
                        return syn::Error::new_spanned(
                            &kv.value,
                            "only string, bool, and int literals are supported as property values",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
                _ => {
                    return syn::Error::new_spanned(
                        &kv.value,
                        "only string, bool, and int literals are supported as property values",
                    )
                    .to_compile_error()
                    .into();
                }
            };
            props.push((key, value));
        }
        let mut matched = vec![];
        'outer: for (id, v) in filtered_names.clone() {
            if let Some(obj) = v.as_object() {
                if let Some(block_props) = obj.get("properties").and_then(|p| p.as_object()) {
                    for (k, v) in block_props.iter() {
                        // Convert to a string
                        let converted = if let Some(s) = v.as_str() {
                            s.to_string()
                        } else if let Some(b) = v.as_bool() {
                            b.to_string()
                        } else if let Some(n) = v.as_i64() {
                            n.to_string()
                        } else {
                            // unsupported property type
                            continue 'outer;
                        };
                        // property is a single string
                        if let Some((_, val)) = props.iter().find(|(key, _)| key == k) {
                            if converted != *val {
                                continue 'outer; // no match
                            }
                        } else {
                            continue 'outer; // property not specified
                        }
                    }
                    matched.push(id);
                } else if !props.is_empty() {
                    continue 'outer; // properties specified but block has none
                } else {
                    matched.push(id); // no properties to match, accept this block
                }
            }
        }
        if matched.len() > 1 {
            syn::Error::new_spanned(
                name_str.clone(),
                format!("block '{}' with specified properties has multiple variants, please refine properties", name_str),
            )
                .to_compile_error()
                .into()
        } else if matched.is_empty() {
            if get_properties_for_id(name_str.clone()).is_empty() {
                syn::Error::new_spanned(
                    name_str.clone(),
                    format!(
                        "block '{}' has no properties but the following properties were given: {}",
                        name_str.clone(),
                        pretty_print_given_props(opts)
                    ),
                )
                .to_compile_error()
                .into()
            } else {
                syn::Error::new_spanned(
                    name_str.clone(),
                    format!(
                        "no variant of block '{}' matches the specified properties. Available properties: {}",
                        name_str.clone(), pretty_print_props(&get_properties_for_id(name_str))
                    ),
                )
                    .to_compile_error()
                    .into()
            }
        } else {
            let res = matched[0];
            return quote! { ferrumc_world::block_id::BlockId($res) };
        }
    } else if filtered_names.len() > 1 {
        syn::Error::new_spanned(
            name_str.clone(),
            format!(
                "block '{}' has multiple variants, please specify properties. Available properties: {}",
                name_str.clone(), pretty_print_props(&get_properties_for_id(name_str))
            ),
        )
        .to_compile_error()
        .into()
    } else if filtered_names.is_empty() {
        syn::Error::new_spanned(
            name.clone(),
            format!("block '{}' not found in blockstates.json", name_str),
        )
        .to_compile_error()
        .into()
    } else {
        let first = filtered_names[0].0;
        return quote! { ferrumc_world::block_id::BlockId($first) };
    }
}

fn get_properties_for_id(id: String) -> Vec<(String, String)> {
    // Iterate all blocks and find all with this as the "name" field
    let mut buf = JSON_FILE.to_vec();
    let v = simd_json::to_owned_value(&mut buf).unwrap();
    let filtered_names = v
        .as_object()
        .unwrap()
        .iter()
        .filter(|v| {
            let name_in_json = v.1.get("name");
            if let Some(resolved_name) = name_in_json {
                resolved_name.as_str() == Some(&id)
            } else {
                false
            }
        })
        .map(|v| (v.0.parse::<u32>().unwrap(), v.1))
        .collect::<Vec<_>>();
    let mut all_props = vec![];
    for (_, v) in filtered_names {
        if let Some(obj) = v.as_object() {
            if let Some(block_props) = obj.get("properties").and_then(|p| p.as_object()) {
                for (k, v) in block_props.iter() {
                    let converted = if let Some(s) = v.as_str() {
                        s.to_string()
                    } else if let Some(b) = v.as_bool() {
                        b.to_string()
                    } else if let Some(n) = v.as_i64() {
                        n.to_string()
                    } else {
                        continue; // unsupported property type
                    };
                    all_props.push((k.clone(), converted));
                }
            }
        }
    }
    all_props
}

fn pretty_print_props(props: &Vec<(String, String)>) -> String {
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
            Expr::Lit(expr_lit) => {
                if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                    lit_str.value()
                } else if let syn::Lit::Bool(lit_bool) = &expr_lit.lit {
                    lit_bool.value.to_string()
                } else if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                    lit_int.base10_digits().to_string()
                } else {
                    "unsupported".to_string()
                }
            }
            _ => "unsupported".to_string(),
        };
        s.push_str(&format!("{}: {}, ", key, value));
    }
    s.trim_end_matches(", ").to_string()
}

fn get_name_from_id(id: u32) -> Option<String> {
    let mut buf = JSON_FILE.to_vec();
    let v = simd_json::to_owned_value(&mut buf).unwrap();
    let filtered_names = v
        .as_object()
        .unwrap()
        .iter()
        .filter(|v| {
            let id_in_json = v.0.parse::<u32>().unwrap();
            id_in_json == id
        })
        .map(|v| v.1)
        .collect::<Vec<_>>();
    if let Some(first) = filtered_names.first() {
        if let Some(obj) = first.as_object() {
            if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                return Some(name.to_string());
            }
        }
    }
    None
}
