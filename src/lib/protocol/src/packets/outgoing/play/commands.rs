use crate::codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};
use crate::ids;
use ferrumc_commands::graph::{CommandGraph, node::CommandNode};
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode, Debug)]
#[packet(id = ids::PLAY_CLIENTBOUND_COMMANDS, state = "play")]
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

impl Default for CommandsPacket {
    fn default() -> Self {
        Self::new()
    }
}
