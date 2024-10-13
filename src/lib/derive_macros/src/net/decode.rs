use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::helpers::StructInfo;

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

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
    
    let StructInfo {
        struct_name,
        impl_generics,
        ty_generics,
        where_clause,
        lifetime: _lifetime,
    } = crate::helpers::extract_struct_info(&input);

    let expanded = quote! {
        // impl ferrumc_net_codec::decode::NetDecode for #name {
        impl #impl_generics ferrumc_net_codec::decode::NetDecode for #struct_name #ty_generics #where_clause {
            fn decode<R: std::io::Read>(reader: &mut R, opts: &ferrumc_net_codec::decode::NetDecodeOpts) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                Ok(Self {
                    #(#decode_fields)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}