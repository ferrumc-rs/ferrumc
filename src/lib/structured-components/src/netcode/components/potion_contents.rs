use crate::netcode::errors::ProtocolViolationError;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct PotionContents {
    pub potion_id: PrefixedOptional<VarInt>,
    pub custom_color: PrefixedOptional<i32>,
    pub custom_effects: LengthPrefixedVec<PotionEffect>,
    pub custom_name: String,
}

//https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Potion_Effect
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct PotionEffect {
    pub effect_id: VarInt,
    pub detail: PotionEffectDetail,
}
#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct PotionEffectDetail {
    pub amplifier: VarInt,
    pub duration: VarInt,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    pub hidden_effect: PrefixedOptional<Box<PotionEffectDetail>>,
}

impl NetEncode for PotionContents {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.potion_id.encode(writer, opts)?;
        self.custom_color.encode(writer, opts)?;
        self.custom_effects.encode(writer, opts)?;
        self.custom_name.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.potion_id.encode_async(writer, opts).await?;
        self.custom_color.encode_async(writer, opts).await?;
        self.custom_effects.encode_async(writer, opts).await?;
        self.custom_name.encode_async(writer, opts).await?;

        Ok(())
    }
}

impl NetEncode for PotionEffect {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.effect_id.encode(writer, opts)?;
        self.detail.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.effect_id.encode_async(writer, opts).await?;
        self.detail.encode_async(writer, opts).await?;

        Ok(())
    }
}

impl NetEncode for PotionEffectDetail {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.amplifier.encode(writer, opts)?;
        self.duration.encode(writer, opts)?;
        self.ambient.encode(writer, opts)?;
        self.show_particles.encode(writer, opts)?;
        self.show_icon.encode(writer, opts)?;
        self.hidden_effect.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        let mut current_effect = Some(self);

        while let Some(effect) = current_effect {
            effect.amplifier.encode_async(writer, opts).await?;
            effect.duration.encode_async(writer, opts).await?;
            effect.ambient.encode_async(writer, opts).await?;
            effect.show_particles.encode_async(writer, opts).await?;
            effect.show_icon.encode_async(writer, opts).await?;

            if let PrefixedOptional::Some(hidden_effect) = &effect.hidden_effect {
                true.encode_async(writer, opts).await?;
                current_effect = Some(hidden_effect);
            } else {
                false.encode_async(writer, opts).await?;
                current_effect = None;
            }
        }

        Ok(())
    }
}

impl NetDecode for PotionContents {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let potion_id: PrefixedOptional<VarInt> = PrefixedOptional::decode(reader, opts)?;
        let custom_color: PrefixedOptional<i32> = PrefixedOptional::decode(reader, opts)?;
        let custom_effects: LengthPrefixedVec<PotionEffect> = LengthPrefixedVec::decode(reader, opts)?;
        let custom_name: String = String::decode(reader, opts)?;

        Ok(PotionContents {
            potion_id,
            custom_color,
            custom_effects,
            custom_name,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let potion_id: PrefixedOptional<VarInt> = PrefixedOptional::decode_async(reader, opts).await?;
        let custom_color: PrefixedOptional<i32> = PrefixedOptional::decode_async(reader, opts).await?;
        let custom_effects: LengthPrefixedVec<PotionEffect> = LengthPrefixedVec::decode_async(reader, opts).await?;
        let custom_name: String = String::decode_async(reader, opts).await?;

        Ok(PotionContents {
            potion_id,
            custom_color,
            custom_effects,
            custom_name,
        })
    }
}

impl NetDecode for PotionEffect {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let effect_id: VarInt = VarInt::decode(reader, opts)?;
        let detail: PotionEffectDetail = PotionEffectDetail::decode(reader, opts)?;

        Ok(PotionEffect { effect_id, detail })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let effect_id: VarInt = VarInt::decode_async(reader, opts).await?;
        let detail: PotionEffectDetail =
            PotionEffectDetail::decode_async(reader, opts).await?;

        Ok(PotionEffect { effect_id, detail })
    }
}

impl NetDecode for PotionEffectDetail {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let amplifier: VarInt = VarInt::decode(reader, opts)?;
        let duration: VarInt = VarInt::decode(reader, opts)?;
        let ambient: bool = bool::decode(reader, opts)?;
        let show_particles: bool = bool::decode(reader, opts)?;
        let show_icon: bool = bool::decode(reader, opts)?;
        let hidden_effect: PrefixedOptional<Box<PotionEffectDetail>> =
            PrefixedOptional::decode(reader, opts)?;

        Ok(PotionEffectDetail {
            amplifier,
            duration,
            ambient,
            show_particles,
            show_icon,
            hidden_effect,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        struct LayerData {
            amplifier: VarInt,
            duration: VarInt,
            ambient: bool,
            show_particles: bool,
            show_icon: bool,
        }

        let mut stack = Vec::new();

        loop {
            let amplifier = VarInt::decode_async(reader, opts).await?;
            let duration = VarInt::decode_async(reader, opts).await?;
            let ambient = bool::decode_async(reader, opts).await?;
            let show_particles = bool::decode_async(reader, opts).await?;
            let show_icon = bool::decode_async(reader, opts).await?;

            stack.push(LayerData {
                amplifier,
                duration,
                ambient,
                show_particles,
                show_icon,
            });

            let has_next = bool::decode_async(reader, opts).await?;

            if !has_next {
                break;
            }
        }

        let mut next_child: PrefixedOptional<Box<PotionEffectDetail>> =
            PrefixedOptional::None;

        while let Some(data) = stack.pop() {
            let current_node = PotionEffectDetail {
                amplifier: data.amplifier,
                duration: data.duration,
                ambient: data.ambient,
                show_particles: data.show_particles,
                show_icon: data.show_icon,
                hidden_effect: next_child,
            };

            next_child = PrefixedOptional::Some(Box::new(current_node));
        }

        match next_child {
            PrefixedOptional::Some(boxed_root) => Ok(*boxed_root),
            PrefixedOptional::None => {
                let protocol_error = ProtocolViolationError("Empty potion effect chain decoded");
                Err(NetDecodeError::ExternalError(Box::new(protocol_error)))
            }
        }
    }
}
