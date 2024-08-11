use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub(crate) fn nbt_serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Struct(data) = input.data else {
        panic!("NBTSerialize can only be derived for structs");
    };

    let fields = data.fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            self.#field_name.serialize(writer)?;
        }
    });

    let serialize_impl = quote! {
        impl ::nbt_lib::nbt_spec::serializer::NBTSerialize for #name {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                #(#fields)*
                Ok(())
            }
        }
    };

    TokenStream::from(serialize_impl)
}

