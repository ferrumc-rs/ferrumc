use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Error, Ident, Lit, LitStr, Result, Token};

pub(crate) mod matches;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

struct Input {
    name: LitStr,
    opts: Option<Opts>,
}

struct Opts {
    pairs: Punctuated<Kv, Token![,]>,
}
#[derive(Debug)]
struct Kv {
    key: Ident,
    _colon: Token![:],
    value: Value,
}

impl Kv {
    fn to_string_pair(&self) -> (String, String) {
        (
            match &self.key.to_string() {
                v if v.starts_with("r#") => v.split_once("r#").unwrap().1.to_string(),
                v => v.clone(),
            },
            match &self.value {
                Value::Static(v) => match v {
                    Lit::Str(v) => v.value(),
                    Lit::Bool(v) => v.value.to_string(),
                    Lit::Int(v) => v.base10_digits().to_string(),
                    _ => unreachable!(),
                },
                Value::Any(_) => "_".into(),
                Value::Ident(v) => v.to_string(),
            },
        )
    }
}
#[derive(Debug)]
enum Value {
    Static(Lit),
    Any(Token![_]),
    Ident(Ident),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![_]) {
            return Ok(Self::Any(input.parse()?));
        }
        if input.peek(Ident) {
            return Ok(Self::Ident(input.parse()?));
        }
        if input.peek(Lit) {
            let lit = input.parse()?;
            if matches!(lit, Lit::Str(_) | Lit::Bool(_) | Lit::Int(_)) {
                return Ok(Self::Static(lit));
            }
        }
        Err(input.error("the property value must be _, ident or literal string, bool or int"))
    }
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

impl quote::ToTokens for Input {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        self.opts.to_tokens(tokens);
    }
}

impl quote::ToTokens for Opts {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.pairs.to_tokens(tokens);
    }
}

impl quote::ToTokens for Kv {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.key.to_tokens(tokens);
        self._colon.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl quote::ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::Static(lit) => lit.to_tokens(tokens),
            Value::Any(underscore) => underscore.to_tokens(tokens),
            Value::Ident(ident) => ident.to_tokens(tokens),
        }
        let first = filtered_names.first().unwrap().0;
        return quote! { BlockStateId::new(#first) }.into();
    };

pub fn block(input: TokenStream) -> TokenStream {
    let out = match syn::parse_macro_input!(input as Input) {
        Input { name, opts: None } => static_block(name),
        Input {
            name,
            opts: Some(opts),
        } => block_with_props(name, opts),
    };
    match out {
        Ok(v) => v,
        Err(v) => v.into_compile_error().into(),
    }
}

fn block_with_props(name: LitStr, opts: Opts) -> Result<TokenStream> {
    let name_value = parse_name(&name);
    let props = PROP_PARTS.get(&name_value);
    if props.is_none() {
        return Err(Error::new_spanned(
            &name,
            format!(
                "the block `{}` is not found in the blockstates.json file (PROP_PARTS is not populated)",
                name_value
            ),
        ));
    }
    if props.is_some_and(|x| x.is_empty()) {
        return Err(Error::new_spanned(
            &name,
            format!("the block `{}` has no properties", name_value),
        ));
    }
    let props = props.unwrap().to_vec();

    let opts_strings = opts
        .pairs
        .iter()
        .map(Kv::to_string_pair)
        .zip(opts.pairs.iter())
        .collect::<Vec<_>>();
    let mut unknown_props = Vec::new();
    let mut missing_props = Vec::new();
    for prop in &props {
        if !opts_strings.iter().any(|((k, _), _)| k == prop) {
            missing_props.push(prop.to_string());
        }
    }
    for ((k, _), _) in &opts_strings {
        if !props.contains(&k.as_str()) {
            unknown_props.push(k.clone());
        }
    }
    if !unknown_props.is_empty() {
        return Err(Error::new_spanned(
            &name,
            format!(
                "the block {name_value} has no properties with names: [{}]",
                unknown_props.join(", ")
            ),
        ));
    }
    if !missing_props.is_empty() {
        return Err(Error::new_spanned(
            name,
            format!(
                "the block `{name_value}` is missing these properties: [{}]",
                missing_props.join(", ")
            ),
        ));
    }

    let mut block_states = BLOCK_STATES
        .get(&name_value)
        .expect(&format!("block name {name_value} should be present"))
        .to_vec();
    for ((k, v), kv) in &opts_strings {
        let property_filter = match &kv.value {
            Value::Static(_) => BLOCK_STATES
                .get(&format!("{k}:{v}"))
                .ok_or(Error::new_spanned(
                    kv,
                    format!(
                        "the value `{v}` is not a valid value for the property `{k}`, available values: [{}]",
                        PROP_PARTS
                            .get(k)
                            .expect("the key `{k}` exists in PROP_PARTS")
                            .join(", ")
                    ),
                ))?
                .to_vec(),
            Value::Any(_) => {
                let vs = PROP_PARTS
                    .get(k)
                    .ok_or(
                        Error::new_spanned(
                            &kv.key,
                            format!("the property `{k}` is not present in the blockstates.json file (PROP_PARTS is not populated)")
                        )
                    )?
                    .iter()
                    .map(|v| BLOCK_STATES
                        .get(&format!("{k}:{v}"))
                        .ok_or(
                            Error::new_spanned(
                                kv,
                                format!("the value `{v}` is not a valid value for the property `{k}`, available values: [{}]", PROP_PARTS
                                    .get(k)
                                    .expect(&format!("the key `{k}` exists in PROP_PARTS")).join(", ")
                                )
                        ))
                    )
                    .collect::<Result<Vec<_>>>()?;
                let mut combined_block_states = Vec::new();
                for bs in vs {
                    combined_block_states = combine(&combined_block_states, bs);
                }
                combined_block_states
            }
            Value::Ident(_) => {
                return Err(Error::new_spanned(&kv.key, "Ident keys are not supported"))
            }
        };
        block_states = intersect(&block_states, &property_filter);
    }

    if block_states.is_empty() {
        return Err(Error::new_spanned(
            Input {
                name,
                opts: Some(opts),
            },
            "no block state corresponds to this combination of properties",
        ));
    }
    let block_ids = block_states.iter().map(|x| *x as u32);
    Ok(quote! {BlockId(#(#block_ids)|*)}.into())
}

fn static_block(name: LitStr) -> Result<TokenStream> {
    let name_value = parse_name(&name);
    let props = PROP_PARTS.get(&name_value);
    if props.is_none() {
        return Err(Error::new_spanned(
            &name,
            format!(
                "the block `{name_value}` not found in blockstates.json (PROP_PARTS is not populated)",
            ),
        ));
    }
    if props.is_some_and(|x| !x.is_empty()) {
        let &props = props.unwrap();
        return Err(Error::new_spanned(
            &name,
            format!(
                "the block `{name_value}` has these properties: [{}], please refine properties",
                props.join(", ")
            ),
        ));
    }
    let block_states = BLOCK_STATES.get(&name_value);
    if block_states.is_none_or(|x| x.is_empty()) {
        return Err(Error::new_spanned(
            &name,
            format!(
                "the block `{name_value}` not found in the blockstates.json file (BLOCK_STATE is not populated)",
            ),
        ));
    }
    let block_states = block_states.unwrap();
    if block_states.len() > 1 {
        return Err(Error::new_spanned(
            name,
            format!(
                "the block `{name_value}` has multiple block states: [{}], but only one expected",
                block_states
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    }

    let id = block_states[0] as u32;
    Ok(quote! { BlockId(#id) }.into())
}

fn parse_name(name: &LitStr) -> String {
    let name_value = name.value();
    match name_value.split_once("minecraft:") {
        Some((_, name)) => name.to_string(),
        None => name_value,
    }
}

fn intersect(a: &[u16], b: &[u16]) -> Vec<u16> {
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

fn combine(a: &[u16], b: &[u16]) -> Vec<u16> {
    let mut v = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                v.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Equal => {
                v.push(a[i]);
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Greater => {
                v.push(b[j]);
                j += 1;
            }
        }
    }
    if i < a.len() || j < b.len() {
        if i < a.len() {
            v.extend_from_slice(&a[i..]);
        } else {
            v.extend_from_slice(&b[j..]);
        }
    }
    v
}

    let res = matched[0];
    quote! { BlockStateId::new(#res) }.into()
}

    quote! { BlockStateId(#id) }.into()
}
