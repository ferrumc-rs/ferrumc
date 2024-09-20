pub(crate) fn get_fields(input: &syn::DeriveInput) -> Vec<&syn::Field> {
    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields.named.iter().collect(),
            syn::Fields::Unnamed(fields) => fields.unnamed.iter().collect(),
            syn::Fields::Unit => vec![],
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => vec![],
    }
}