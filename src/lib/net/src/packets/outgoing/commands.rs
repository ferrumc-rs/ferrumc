use ferrumc_commands::graph::{node::CommandNode, CommandGraph};
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};
use std::io::Write;

#[derive(NetEncode, Debug)]
#[packet(packet_id = "commands", state = "play")]
pub struct CommandsPacket {
    pub graph: LengthPrefixedVec<CommandNode>,
    pub root_idx: VarInt,
}

impl CommandsPacket {
    pub fn new_with(graph: CommandGraph) -> Self {
        Self {
            graph: LengthPrefixedVec::new(graph.nodes),
            root_idx: VarInt::new(0),
        }
    }

    pub fn new() -> Self {
        Self::new_with(ferrumc_commands::infrastructure::get_graph())
    }
}