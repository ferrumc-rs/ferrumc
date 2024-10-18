use proc_macro2::{TokenStream};
use quote::quote;
use syn::{DeriveInput, GenericParam, Type};

/// Retrieves the fields from a struct in a `DeriveInput`.
///
/// # Arguments
///
/// * `input` - A reference to a `syn::DeriveInput` representing the struct.
///
/// # Returns
///
/// A vector of references to the struct's fields.
///
/// # Panics
///
/// This function will panic if the input is not a struct with named fields.
pub(crate) fn get_fields(input: &syn::DeriveInput) -> Vec<&syn::Field> {
    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields.named.iter().collect(),
            syn::Fields::Unnamed(fields) => fields.unnamed.iter().collect(),
            syn::Fields::Unit => Vec::new(),
        },
        syn::Data::Enum(data_enum) => data_enum
            .variants
            .iter()
            .flat_map(|variant| match &variant.fields {
                syn::Fields::Named(fields_named) => fields_named.named.iter().collect::<Vec<_>>(),
                syn::Fields::Unnamed(fields_unnamed) => {
                    fields_unnamed.unnamed.iter().collect::<Vec<_>>()
                }
                syn::Fields::Unit => Vec::new(),
            })
            .collect(),
        syn::Data::Union(_) => panic!("Unions are not supported!"),
    }
}

/// Checks if the given type is an `Option<T>`.
///
/// # Arguments
///
/// * `ty` - A reference to a `syn::Type` to check.
///
/// # Returns
///
/// An `Option` containing a reference to the inner type `T` if `ty` is `Option<T>`,
/// or `None` if `ty` is not an `Option`.
pub(crate) fn is_field_type_optional(field: &syn::Field) -> bool {
    let Type::Path(type_path) = &field.ty else {
        return false;
    };

    type_path
        .path
        .segments
        .iter()
        .any(|segment| segment.ident.to_string().to_lowercase() == "option")
}


pub struct StructInfo<'a> {
    pub struct_name: &'a syn::Ident,
    pub impl_generics: syn::ImplGenerics<'a>,
    pub ty_generics: syn::TypeGenerics<'a>,
    pub where_clause: Option<&'a syn::WhereClause>,
    pub lifetime: TokenStream,
}

pub(crate) fn extract_struct_info(input: &DeriveInput) -> StructInfo {
    let struct_name = &input.ident;
    let impl_generics = input.generics.clone();
    let ty_generics = input.generics.split_for_impl().1;
    let where_clause = &input.generics.where_clause;

    /*// Introduce a new lifetime 'de for deserialization
    let has_lifetime = impl_generics.params.iter().any(|param| matches!(param, GenericParam::Lifetime(_)));
    /*if !has_lifetime {
        let de_lifetime = Lifetime::new("'de", Span::call_site());
        impl_generics.params.insert(
            0,
            GenericParam::Lifetime(LifetimeParam::new(de_lifetime.clone())),
        );
    }*/*/

    let lifetime = impl_generics
        .params
        .iter()
        .find_map(|param| match param {
            GenericParam::Lifetime(lifetime) => Some(lifetime.lifetime.clone()),
            _ => None,
        });

    let lifetime = match lifetime {
        Some(lifetime) => quote! { <#lifetime> },
        None => quote! { },
    };

    let (impl_generics, _, _) = input.generics.split_for_impl();

    StructInfo {
        struct_name,
        impl_generics: impl_generics.clone(),
        ty_generics,
        where_clause: where_clause.as_ref(),
        lifetime,
    }
}

pub (crate) fn get_derive_attributes(input: &DeriveInput, path_name: &str) -> Vec<syn::Attribute> {
    input.attrs.iter().filter(|attr| attr.path().is_ident(path_name)).cloned().collect()
}