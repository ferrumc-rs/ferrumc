use syn::Type;

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
            syn::Fields::Unit => panic!("Unit structs are not supported!"),
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("Only structs are supported!"),
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
    
    type_path.path.segments.iter().any(|segment| segment.ident.to_string().to_lowercase() == "option")
}