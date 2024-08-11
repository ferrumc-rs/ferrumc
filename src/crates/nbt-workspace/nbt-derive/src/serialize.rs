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
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        let field_type = &f.ty;
        quote! {
            // Serialize for #field_name
            {
/*  TAG_TYPE  */<#field_type as nbt_lib::nbt_spec::impls::NBTTag>::tag_type().serialize(writer)?;
/*(l+)TAG_NAME*/#field_name_as_string.serialize(writer)?;
/*ACTUAL_VALUE*/self.#field_name.serialize(writer)?;
            }
        }
    });

    let serialize_impl = quote! {
        impl ::nbt_lib::nbt_spec::serializer::NBTSerialize for #name {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> ::nbt_lib::NBTResult<()> {
                #(#fields)*
                nbt_lib::nbt_spec::tag_types::TAG_END.serialize(writer)?;
                Ok(())
            }
        }

        impl nbt_lib::nbt_spec::impls::NBTTag for #name {
            fn tag_type() -> u8 {
                nbt_lib::nbt_spec::tag_types::TAG_COMPOUND
            }
        }
    };

    TokenStream::from(serialize_impl)
}

