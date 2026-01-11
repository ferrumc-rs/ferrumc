use crate::structured_components::data::block_predicate::ExactDataComponentMatcher;
use crate::structured_components::data::block_predicate::PartialDataComponentMatcher;
use crate::structured_components::data::block_predicate::Property;
use crate::structured_components::data::{utils, IdSet};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use std::io::Read;
use tokio::io::AsyncRead;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct BlockPredicate {
    pub blocks: PrefixedOptional<IdSet>,
    pub properties: PrefixedOptional<LengthPrefixedVec<Property>>,
    pub nbt: PrefixedOptional<Vec<u8>>,
    pub data_components: LengthPrefixedVec<ExactDataComponentMatcher>,
    pub partial_data_component_predicates: LengthPrefixedVec<PartialDataComponentMatcher>,
}

impl NetDecode for BlockPredicate {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        Ok(BlockPredicate {
            blocks: PrefixedOptional::decode(reader, opts)?,
            properties: PrefixedOptional::decode(reader, opts)?,
            nbt: if bool::decode(reader, opts)? {
                PrefixedOptional::Some(utils::decode_compound_to_buffer(reader)?)
            } else {
                PrefixedOptional::None
            },
            data_components: LengthPrefixedVec::decode(reader, opts)?,
            partial_data_component_predicates: LengthPrefixedVec::decode(reader, opts)?,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        Ok(BlockPredicate {
            blocks: PrefixedOptional::decode_async(reader, opts).await?,
            properties: PrefixedOptional::decode_async(reader, opts).await?,
            nbt: if bool::decode_async(reader, opts).await? {
                PrefixedOptional::Some(utils::decode_compound_to_buffer_async(reader).await?)
            } else {
                PrefixedOptional::None
            },
            data_components: LengthPrefixedVec::decode_async(reader, opts).await?,
            partial_data_component_predicates: LengthPrefixedVec::decode_async(reader, opts)
                .await?,
        })
    }
}
