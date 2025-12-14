#![allow(clippy::all)]
#![allow(warnings)]
#![recursion_limit = "256"]
#![feature(async_fn_in_trait)]

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use std::io::Write;

struct ComponentDef {
    variant: &'static str,
    id: i32,
    struct_path: Option<&'static str>,
}

pub const OUT_DIR: &str = "src/netcode/generated";
pub const OUT_FILE: &str = "structured_components.rs";

pub fn main() {
    println!("Running structured-components/build/build.rs...");

    let path = std::path::Path::new(OUT_DIR);

    if !path.exists() {
        let _ = fs::create_dir(OUT_DIR);
    }

    let unformatted_code_stream = build();

    let unformatted_code = unformatted_code_stream.to_string();
    let formatted_code = format_code(&unformatted_code);
    write_generated_file(&formatted_code, OUT_FILE);
}

fn build() -> TokenStream {
    let components = vec![
        ComponentDef { id: 2, variant: "MaxDamage", struct_path: Some("crate::netcode::components::damage::MaxDamage") },
        ComponentDef { id: 3, variant: "Damage", struct_path: Some("crate::netcode::components::damage::Damage") },
        ComponentDef { id: 4, variant: "Unbreakable", struct_path: None },
        ComponentDef { id: 10, variant: "Enchantments", struct_path: Some("crate::netcode::components::enchantments::enchantments_collection::EnchantmentsCollection") },
        ComponentDef { id: 18, variant: "EnchantmentGlintOverride", struct_path: Some("crate::netcode::components::enchantments::enchantment_glint_override::EnchantmentGlintOverride") },
        ComponentDef { id: 27, variant: "Enchantable", struct_path: Some("crate::netcode::components::enchantments::enchantable::Enchantable") },
        ComponentDef { id: 34, variant: "StoredEnchantments", struct_path: Some("crate::netcode::components::enchantments::enchantments_collection::EnchantmentsCollection") },
        ComponentDef { id: 42, variant: "PotionContents", struct_path: Some("crate::netcode::components::potion_contents::PotionContents") },
        ComponentDef { id: 44, variant: "SuspiciousStewEffects", struct_path: Some("crate::netcode::components::suspicious_stew_effects::SuspiciousStewEffects") },
        ComponentDef { id: 54, variant: "OminousBottleAmplifier", struct_path: Some("crate::netcode::components::ominous_bottle_amplifier::OminousBottleAmplifier") },
        ComponentDef { id: 60, variant: "Fireworks", struct_path: Some("crate::netcode::components::fireworks::Fireworks") },
    ];

    let enum_variants = generate_enum_variants(&components);
    let to_id_match_arms = generate_to_id_arms(&components);
    let decode_match_arms = generate_decode_arms(&components, false);
    let decode_async_match_arms = generate_decode_arms(&components, true);
    let encode_match_arms = generate_encode_arms(&components, false);
    let encode_async_match_arms = generate_encode_arms(&components, true);

    quote! {
        use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
        use ferrumc_net_codec::decode::errors::NetDecodeError;
        use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
        use ferrumc_net_codec::encode::errors::NetEncodeError;
        use ferrumc_net_codec::net_types::var_int::VarInt;
        use std::io::{Read, Write};
        use tokio::io::{AsyncRead, AsyncWrite};
        use crate::netcode::errors::{InvalidStructuredComponentEnumError, NotSupportedStructuredComponentError};

        #[derive(Debug, Clone, Hash, Default, PartialEq)]
        pub enum StructuredComponent {
            #[default]
            Invalid,
            #enum_variants
        }

        impl StructuredComponent {
            pub fn to_id(&self) -> Result<VarInt, InvalidStructuredComponentEnumError> {
                match self {
                    StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError()),
                    #to_id_match_arms
                }
            }
        }

        impl NetEncode for StructuredComponent {
            fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
                if let StructuredComponent::Invalid = self {
                     return Err(InvalidStructuredComponentEnumError().into());
                }

                let id = self.to_id().map_err(|e| NetEncodeError::from(e))?;
                id.encode(writer, opts)?;

                match self {
                    StructuredComponent::Invalid => unreachable!(),
                    #encode_match_arms
                }
            }

            async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
                if let StructuredComponent::Invalid = self {
                     return Err(InvalidStructuredComponentEnumError().into());
                }

                let id = self.to_id().map_err(|e| NetEncodeError::from(e))?;
                id.encode_async(writer, opts).await?;

                match self {
                    StructuredComponent::Invalid => unreachable!(),
                    #encode_async_match_arms
                }
            }
        }

        impl NetDecode for StructuredComponent {
            fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
                let id = VarInt::decode(reader, opts)?;

                let _probably_data_length = VarInt::decode(reader, opts)?;

                match id.0 {
                    #decode_match_arms
                    _ => {
                        Err(NotSupportedStructuredComponentError(id).into())
                    }
                }
            }

            async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
                let id = VarInt::decode_async(reader, opts).await?;

                let _probably_data_length = VarInt::decode_async(reader, opts).await?;

                match id.0 {
                    #decode_async_match_arms
                    _ => {
                        Err(NotSupportedStructuredComponentError(id).into())
                    }
                }
            }
        }
    }
}

fn generate_enum_variants(components: &[ComponentDef]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for comp in components {
        let variant = format_ident!("{}", comp.variant);
        if let Some(path_str) = comp.struct_path {
            let path: syn::Path = syn::parse_str(path_str).unwrap();
            tokens.extend(quote! {
                #variant(#path),
            });
        } else {
            tokens.extend(quote! {
                #variant,
            });
        }
    }
    tokens
}

fn generate_to_id_arms(components: &[ComponentDef]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for comp in components {
        let variant = format_ident!("{}", comp.variant);
        let id = comp.id;
        if comp.struct_path.is_some() {
            tokens.extend(quote! {
                StructuredComponent::#variant(_) => Ok(VarInt::from(#id)),
            });
        } else {
            tokens.extend(quote! {
                StructuredComponent::#variant => Ok(VarInt::from(#id)),
            });
        }
    }
    tokens
}

fn generate_decode_arms(components: &[ComponentDef], is_async: bool) -> TokenStream {
    let mut tokens = TokenStream::new();
    for comp in components {
        let variant = format_ident!("{}", comp.variant);
        let id = comp.id;

        if let Some(path_str) = comp.struct_path {
            let path: syn::Path = syn::parse_str(path_str).unwrap();
            if is_async {
                tokens.extend(quote! {
                    #id => {
                        Ok(StructuredComponent::#variant(#path::decode_async(reader, opts).await?))
                    },
                });
            } else {
                tokens.extend(quote! {
                    #id => {
                        Ok(StructuredComponent::#variant(#path::decode(reader, opts)?))
                    },
                });
            }
        } else {
            tokens.extend(quote! {
                #id => {
                    Ok(StructuredComponent::#variant)
                },
            });
        }
    }
    tokens
}

fn generate_encode_arms(components: &[ComponentDef], is_async: bool) -> TokenStream {
    let mut tokens = TokenStream::new();
    for comp in components {
        let variant = format_ident!("{}", comp.variant);

        if comp.struct_path.is_some() {
            if is_async {
                tokens.extend(quote! {
                    StructuredComponent::#variant(inner) => inner.encode_async(writer, opts).await,
                });
            } else {
                tokens.extend(quote! {
                    StructuredComponent::#variant(inner) => inner.encode(writer, opts),
                });
            }
        } else {
            // Unit struct - nothing to encode
            tokens.extend(quote! {
                StructuredComponent::#variant => Ok(()),
            });
        }
    }
    tokens
}

//stole it from data/build:

pub fn write_generated_file(new_code: &str, out_file: &str) {
    let path = std::path::Path::new(OUT_DIR).join(out_file);

    if path.exists() {
        if let Ok(existing_code) = fs::read_to_string(&path) {
            if existing_code == new_code {
                return; // No changes, so we skip writing.
            }
        }
    }

    fs::write(&path, new_code)
        .unwrap_or_else(|_| panic!("Failed to write to file: {}", path.display()));
}

pub fn format_code(unformatted_code: &str) -> String {
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt process.");

    child
        .stdin
        .take()
        .expect("Failed to take rustfmt stdin")
        .write_all(unformatted_code.as_bytes())
        .expect("Failed to write to rustfmt stdin.");

    let output = child
        .wait_with_output()
        .expect("Failed to wait for rustfmt process.");

    if output.status.success() {
        String::from_utf8(output.stdout).expect("rustfmt output was not valid UTF-8.")
    } else {
        panic!(
            "rustfmt failed with status: {}\n--- stderr ---\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
