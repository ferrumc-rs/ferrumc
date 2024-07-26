use syn::parse::{Parse, ParseStream};

#[derive(Default, Debug)]
pub struct FieldAttrs {
    pub rename: Option<String>,
    pub flatten: bool,
}

#[derive(Default, Debug)]
pub struct UnitAttrs {
    pub rename: Option<String>,
}

#[derive(Default, Debug)]
pub struct StructAttrs {
    pub deny_unknown_fields: bool,
}

impl Parse for FieldAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();

        while !input.is_empty() {
            let attr = input.parse::<proc_macro2::Ident>()?;
            match attr.to_string().as_str() {
                "rename" => {
                    input.parse::<syn::Token![=]>()?;
                    let rename = input.parse::<syn::LitStr>()?;

                    attrs.rename = Some(rename.value());
                }
                "flatten" => {
                    attrs.flatten = true;
                }
                _ => todo!(),
            }
        }

        Ok(attrs)
    }
}

impl Parse for UnitAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();

        while !input.is_empty() {
            let attr = input.parse::<proc_macro2::Ident>()?;
            match attr.to_string().as_str() {
                "rename" => {
                    input.parse::<syn::Token![=]>()?;
                    let rename = input.parse::<syn::LitStr>()?;

                    attrs.rename = Some(rename.value());
                }
                _ => todo!(),
            }
        }

        Ok(attrs)
    }
}

impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();

        while !input.is_empty() {
            let attr = input.parse::<proc_macro2::Ident>()?;
            match attr.to_string().as_str() {
                "deny_unknown_fields" => {
                    attrs.deny_unknown_fields = true;
                }
                _ => todo!(),
            }
        }

        Ok(attrs)
    }
}

pub fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut field_attrs = FieldAttrs::default();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("simdnbt")) {
        let new_attr = attr
            .parse_args::<FieldAttrs>()
            .expect("invalid simdnbt attr");
        if let Some(rename) = new_attr.rename {
            field_attrs.rename = Some(rename);
        }
        if new_attr.flatten {
            field_attrs.flatten = true;
        }
    }

    field_attrs
}

pub fn parse_unit_attrs(attrs: &[syn::Attribute]) -> UnitAttrs {
    let mut unit_attrs = UnitAttrs::default();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("simdnbt")) {
        let new_attr = attr
            .parse_args::<UnitAttrs>()
            .expect("invalid simdnbt attr");
        if let Some(rename) = new_attr.rename {
            unit_attrs.rename = Some(rename);
        }
    }

    unit_attrs
}

pub fn parse_struct_attrs(attrs: &[syn::Attribute]) -> StructAttrs {
    let mut struct_attrs = StructAttrs::default();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("simdnbt")) {
        let new_attr = attr
            .parse_args::<StructAttrs>()
            .expect("invalid simdnbt attr");
        if new_attr.deny_unknown_fields {
            struct_attrs.deny_unknown_fields = true;
        }
    }

    struct_attrs
}
