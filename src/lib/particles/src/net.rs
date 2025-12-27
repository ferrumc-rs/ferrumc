use crate::{ParticleType, VibrationSource};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use ParticleType::*;

impl NetEncode for ParticleType {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt::new(self.discriminant()).encode(writer, opts)?;
        match self {
            Block { blockstate } | BlockMarker { blockstate } | FallingDust { blockstate } => {
                blockstate.to_varint().encode(writer, opts)
            }
            DragonBreath { power } => {
                writer.write_all(&power.to_le_bytes())?;
                Ok(())
            }
            Dust { color, scale } => {
                color.to_i32().encode(writer, opts)?;
                writer.write_all(&scale.to_le_bytes())?;
                Ok(())
            }
            DustColorTransition { from, to, scale } => {
                from.to_i32().encode(writer, opts)?;
                to.to_i32().encode(writer, opts)?;
                writer.write_all(&scale.to_le_bytes())?;
                Ok(())
            }
            Effect { color, power } => {
                color.to_i32().encode(writer, opts)?;
                writer.write_all(&power.to_le_bytes())?;
                Ok(())
            }
            EntityEffect { color } => color.to_i32().encode(writer, opts),
            TintedLeaves { color } => color.to_i32().encode(writer, opts),
            SculkCharge { roll } => {
                writer.write_all(&roll.to_le_bytes())?;
                Ok(())
            }
            Flash { color } => color.to_i32().encode(writer, opts),
            InstantEffect { color, power } => {
                color.to_i32().encode(writer, opts)?;
                writer.write_all(&power.to_le_bytes())?;
                Ok(())
            }
            Item { item } => item.encode(writer, opts),
            Vibration { source, ticks } => {
                source.encode(writer, opts)?;
                ticks.encode(writer, opts)
            }
            Trail {
                x,
                y,
                z,
                color,
                duration,
            } => {
                writer.write_all(&x.to_le_bytes())?;
                writer.write_all(&y.to_le_bytes())?;
                writer.write_all(&z.to_le_bytes())?;
                color.to_i32().encode(writer, opts)?;
                duration.encode(writer, opts)
            }
            Shriek { delay } => delay.encode(writer, opts),
            DustPillar { blockstate } => blockstate.to_varint().encode(writer, opts),
            BlockCrumble { blockstate } => blockstate.to_varint().encode(writer, opts),

            _ => Ok(()),
        }
    }
    async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        _: &mut W,
        _: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        unreachable!()
    }
}

impl NetEncode for &VibrationSource {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            VibrationSource::Block { position } => {
                VarInt(0).encode(writer, opts)?;
                NetworkPosition::from(*position).encode(writer, opts)
            }
            VibrationSource::Entity {
                entity_id,
                eye_height,
            } => {
                VarInt(1).encode(writer, opts)?;
                entity_id.encode(writer, opts)?;
                writer.write_all(&eye_height.to_le_bytes())?;
                Ok(())
            }
        }
    }
    async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        _: &mut W,
        _: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        unreachable!()
    }
}
