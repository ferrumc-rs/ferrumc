use proc_macro::TokenStream;

use syn::Data;

pub fn derive(input: TokenStream) -> TokenStream {
    // generate auto-implemented constructor
    // $type::new($($field: expr),*)

    // parse input
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // get the name of the struct
    let name = &input.ident;

    // get the fields of the struct
    let fields = match input.data {
        Data::Struct(data) => {
            data.fields
        }
        Data::Enum(_) => {
            panic!("Cannot derive Constructor for enums")
        }
        Data::Union(_) => {
            panic!("Cannot derive Constructor for unions")
        }
    };

    // generate the constructor
    let constructor = generate_constructor(name, fields);

    // return the generated constructor
    constructor.into()
}

fn generate_constructor(name: &syn::Ident, fields: syn::Fields) -> proc_macro2::TokenStream {
    match fields {
        syn::Fields::Named(fields) => {
            let fields = fields.named.into_iter().map(|field| {
                let field_name = field.ident.unwrap();
                quote! { #field_name }
            })
            todo!()
        }
        _ => {
            panic!("Cannot derive Constructor for structs with unnamed fields or unit structs")
        }
    }
}