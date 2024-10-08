use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        unimplemented!("NetDecode can only be derived for structs");
    };

    let decode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            #field_name: <#field_ty as ferrumc_net_codec::decode::NetDecode>::decode(reader, opts)?,
        }
    });

    let expanded = quote! {
        impl ferrumc_net_codec::decode::NetDecode for #name {
            fn decode<R: std::io::Read>(reader: &mut R, opts: &ferrumc_net_codec::decode::NetDecodeOpts) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                Ok(Self {
                    #(#decode_fields)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}