use std::io::{Read, Write};

use ferrumc_net_codec::{
    decode::{errors::NetDecodeError, NetDecode, NetDecodeOpts, NetDecodeResult},
    encode::{NetEncode, NetEncodeOpts, NetEncodeResult},
    net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt},
};
use tokio::io::AsyncWrite;

#[derive(Debug, Clone, Copy)]
pub enum SlotComponent {
    MaxStackSize { max_stack_size: VarInt },
}

impl NetEncode for SlotComponent {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        VarInt::new(self.get_type()).encode(writer, opts)?;
        match self {
            SlotComponent::MaxStackSize { max_stack_size } => {
                max_stack_size.encode(writer, opts)?;
            }
        };

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        VarInt::new(self.get_type())
            .encode_async(writer, opts)
            .await?;
        match self {
            SlotComponent::MaxStackSize { max_stack_size } => {
                max_stack_size.encode_async(writer, opts).await?;
            }
        };

        Ok(())
    }
}

impl NetDecode for SlotComponent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let id = VarInt::decode(reader, opts)?;
        match *id {
            1 => Ok(SlotComponent::MaxStackSize {
                max_stack_size: VarInt::decode(reader, opts)?,
            }),
            _ => Err(NetDecodeError::InvalidEnumVariant),
        }
    }
}

impl SlotComponent {
    pub fn get_type(&self) -> i32 {
        match self {
            SlotComponent::MaxStackSize { .. } => 1,
        }
    }
}

#[derive(Debug)]
pub struct NetworkSlot {
    pub item_count: VarInt,
    pub item_id: Option<VarInt>,
    pub components_to_add: Option<LengthPrefixedVec<SlotComponent>>,
    pub components_to_remove: Option<LengthPrefixedVec<VarInt>>,
}

impl NetEncode for NetworkSlot {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        self.item_count.encode(writer, opts)?;

        if let Some(id) = self.item_id {
            id.encode(writer, opts)?;
        }

        match (&self.components_to_add, &self.components_to_remove) {
            (Some(components_add), Some(components_remove)) => {
                components_add.length.encode(writer, opts)?;
                components_remove.length.encode(writer, opts)?;

                for component in &components_add.data {
                    component.encode(writer, opts)?;
                }

                for component in &components_remove.data {
                    component.encode(writer, opts)?;
                }
            }
            _ => {} // Do nothing if both components are None
        }

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        self.item_count.encode_async(writer, opts).await?;

        if let Some(id) = self.item_id {
            id.encode_async(writer, opts).await?;
        }

        match (&self.components_to_add, &self.components_to_remove) {
            (Some(components_add), Some(components_remove)) => {
                components_add.length.encode_async(writer, opts).await?;
                components_remove.length.encode_async(writer, opts).await?;

                for component in &components_add.data {
                    component.encode_async(writer, opts).await?;
                }

                for component in &components_remove.data {
                    component.encode_async(writer, opts).await?;
                }
            }
            _ => {} // Do nothing if both components are None
        }

        Ok(())
    }
}

impl NetDecode for NetworkSlot {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let item_count = VarInt::decode(reader, opts)?;

        if item_count == 0 {
            return Ok(NetworkSlot::empty());
        }

        let item_id = VarInt::decode(reader, opts)?;

        let mut components_to_add = Vec::with_capacity(*VarInt::decode(reader, opts)? as usize);
        let mut components_to_remove = Vec::with_capacity(*VarInt::decode(reader, opts)? as usize);

        for _ in 0..components_to_add.capacity() {
            components_to_add.push(SlotComponent::decode(reader, opts)?);
        }

        for _ in 0..components_to_remove.capacity() {
            components_to_remove.push(VarInt::decode(reader, opts)?);
        }

        Ok(Self {
            item_count,
            item_id: Some(item_id),
            components_to_add: if components_to_add.is_empty() {
                None
            } else {
                Some(LengthPrefixedVec::new(components_to_add))
            },
            components_to_remove: if components_to_remove.is_empty() {
                None
            } else {
                Some(LengthPrefixedVec::new(components_to_remove))
            },
        })
    }
}

impl NetworkSlot {
    pub fn new(item_count: i32, item_id: i32) -> Self {
        Self::with_components(item_count, item_id, vec![])
    }

    pub fn with_components(item_count: i32, item_id: i32, components: Vec<SlotComponent>) -> Self {
        Self {
            item_count: VarInt::new(item_count),
            item_id: if item_count == 0 {
                None
            } else {
                Some(VarInt::new(item_id))
            },
            components_to_add: if item_count == 0 {
                None
            } else {
                Some(LengthPrefixedVec::new(components))
            },
            components_to_remove: if item_count == 0 {
                None
            } else {
                Some(LengthPrefixedVec::default())
            },
        }
    }

    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn item_id(&mut self, item_id: VarInt) -> &mut Self {
        self.item_id = Some(item_id);
        self
    }
}
