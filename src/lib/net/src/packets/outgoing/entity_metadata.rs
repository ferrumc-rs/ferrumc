// https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Entity_metadata#Entity_Metadata_Format
use crate::packets::outgoing::entity_metadata::entity_state::{EntityState, EntityStateMask};
use crate::packets::outgoing::entity_metadata::index_type::EntityMetadataIndexType;
use crate::packets::outgoing::entity_metadata::value::EntityMetadataValue;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

/// Packet for sending entity metadata updates to clients
#[derive(NetEncode, Clone)]
#[packet(packet_id = "set_entity_data", state = "play")]
pub struct EntityMetadataPacket {
    entity_id: VarInt,
    metadata: Vec<EntityMetadata>,
    terminator: u8, // Always 0xFF to indicate end of metadata (Simple is Best)
}

impl EntityMetadataPacket {
    /// Creates a new metadata packet for the specified entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity ID to update metadata for
    /// * `metadata` - Iterator of metadata entries to send
    ///
    /// # Example
    /// ```ignored
    /// let entity_id = ...;
    /// let metadata = vec![
    ///                     EntityMetadata::entity_sneaking_pressed(),
    ///                     EntityMetadata::entity_sneaking_visual(),
    ///                     EntityMetadata::entity_standing()
    ///                   ];
    /// let packet = EntityMetadataPacket::new(entity_id, metadata);
    /// ```
    pub fn new<T>(entity_id: VarInt, metadata: T) -> Self
    where
        T: IntoIterator<Item = EntityMetadata>,
    {
        Self {
            entity_id,
            metadata: metadata.into_iter().collect(),
            terminator: 0xFF,
        }
    }
}

/// Single metadata entry containing an index, type and value
#[derive(NetEncode, Clone)]
pub struct EntityMetadata {
    index: u8,
    index_type: EntityMetadataIndexType,
    value: EntityMetadataValue,
}

pub mod constructors {
    use super::*;
    use crate::packets::outgoing::entity_metadata::extra_data_types::EntityPose;

    impl EntityMetadata {
        fn new(index_type: EntityMetadataIndexType, value: EntityMetadataValue) -> Self {
            EntityMetadata {
                index: value.index(),
                index_type,
                value,
            }
        }
        /// To hide the name tag and stuff
        pub fn entity_sneaking_pressed() -> Self {
            Self::new(
                EntityMetadataIndexType::Byte,
                EntityMetadataValue::Entity0(EntityStateMask::from_state(
                    EntityState::SneakingVisual,
                )),
            )
        }
        /// Actual sneaking visual, so you can see the player sneaking
        pub fn entity_sneaking_visual() -> Self {
            Self::new(
                EntityMetadataIndexType::Pose,
                EntityMetadataValue::Entity6(EntityPose::Sneaking),
            )
        }

        /// Entity in standing pose
        pub fn entity_standing() -> Self {
            Self::new(
                EntityMetadataIndexType::Pose,
                EntityMetadataValue::Entity6(EntityPose::Standing),
            )
        }
    }
}

mod index_type {
    use super::*;

    /// Available metadata field types
    /// See: https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Entity_metadata#Entity_Metadata_Format
    #[derive(Debug, Clone, Copy)]
    pub enum EntityMetadataIndexType {
        Byte, // (0) Used for bit masks and small numbers
        Pose, // (21) Used for entity pose
    }

    impl EntityMetadataIndexType {
        pub fn index(&self) -> VarInt {
            use EntityMetadataIndexType::*;
            let val = match self {
                Byte => 0,
                Pose => 21,
            };

            VarInt::new(val)
        }
    }

    impl NetEncode for EntityMetadataIndexType {
        fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
            self.index().encode(writer, opts)
        }

        async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
            &self,
            writer: &mut W,
            opts: &NetEncodeOpts,
        ) -> NetEncodeResult<()> {
            self.index().encode_async(writer, opts).await
        }
    }
}

mod value {
    use super::*;
    use crate::packets::outgoing::entity_metadata::extra_data_types::EntityPose;
    /// Possible metadata values that can be sent
    ///
    /// Couldn't be arsed coming up with the names.
    /// Read here:
    /// https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Entity_metadata#Entity
    ///
    /// Formatted like:
    /// {Class Name}{Index}
    #[derive(NetEncode, Clone)]
    pub enum EntityMetadataValue {
        Entity0(EntityStateMask),
        Entity6(EntityPose),
    }

    impl EntityMetadataValue {
        pub fn index(&self) -> u8 {
            use EntityMetadataValue::*;
            match self {
                Entity0(_) => 0,
                Entity6(_) => 6,
            }
        }
    }
}

mod entity_state {
    use ferrumc_macros::NetEncode;
    use std::io::Write;

    /// Bit mask for various entity states
    #[derive(Debug, NetEncode, Clone)]
    pub struct EntityStateMask {
        mask: u8,
    }

    impl Default for EntityStateMask {
        fn default() -> Self {
            Self::new()
        }
    }

    impl EntityStateMask {
        pub fn new() -> Self {
            Self { mask: 0 }
        }

        pub fn from_state(state: EntityState) -> Self {
            let mut mask = Self::new();
            mask.set(state);
            mask
        }

        pub fn set(&mut self, state: EntityState) {
            self.mask |= state.mask();
        }
    }

    /// Individual states that can be applied to an entity
    /// Multiple states can be combined using a bit mask
    #[expect(dead_code)]
    pub enum EntityState {
        OnFire,           // 0x01
        SneakingVisual,   // 0x02
        Sprinting,        // 0x08
        Swimming,         // 0x10
        Invisible,        // 0x20
        Glowing,          // 0x40
        FlyingWithElytra, // 0x80
    }

    impl EntityState {
        pub fn mask(&self) -> u8 {
            use EntityState::*;
            match self {
                OnFire => 0x01,
                SneakingVisual => 0x02,
                Sprinting => 0x08,
                Swimming => 0x10,
                Invisible => 0x20,
                Glowing => 0x40,
                FlyingWithElytra => 0x80,
            }
        }
    }
}

mod extra_data_types {
    use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
    use ferrumc_net_codec::net_types::var_int::VarInt;
    use std::io::Write;
    // STANDING = 0, FALL_FLYING = 1, SLEEPING = 2, SWIMMING = 3, SPIN_ATTACK = 4, SNEAKING = 5, LONG_JUMPING = 6, DYING = 7, CROAKING = 8,
    // USING_TONGUE = 9, SITTING = 10, ROARING = 11, SNIFFING = 12, EMERGING = 13, DIGGING = 14, (1.21.3: SLIDING = 15, SHOOTING = 16,
    // INHALING = 17

    /// Possible poses/animations an entity can have
    #[derive(Debug, Clone)]
    #[expect(dead_code)]
    pub enum EntityPose {
        Standing,
        FallFlying,
        Sleeping,
        Swimming,
        SpinAttack,
        Sneaking,
        LongJumping,
        Dying,
        Croaking,
        UsingTongue,
        Sitting,
        Roaring,
        Sniffing,
        Emerging,
        Digging,
        Sliding,
        Shooting,
        Inhaling,
    }

    impl EntityPose {
        pub fn index(&self) -> VarInt {
            use EntityPose::*;
            let val = match self {
                Standing => 0,
                FallFlying => 1,
                Sleeping => 2,
                Swimming => 3,
                SpinAttack => 4,
                Sneaking => 5,
                LongJumping => 6,
                Dying => 7,
                Croaking => 8,
                UsingTongue => 9,
                Sitting => 10,
                Roaring => 11,
                Sniffing => 12,
                Emerging => 13,
                Digging => 14,
                Sliding => 15,
                Shooting => 16,
                Inhaling => 17,
            };
            VarInt::new(val)
        }
    }

    impl NetEncode for EntityPose {
        fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
            self.index().encode(writer, opts)
        }

        async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
            &self,
            writer: &mut W,
            opts: &NetEncodeOpts,
        ) -> NetEncodeResult<()> {
            self.index().encode_async(writer, opts).await
        }
    }
}
