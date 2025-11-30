use ferrumc_commands::graph::{node::CommandNode, CommandGraph};
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};

#[derive(NetEncode, Debug)]
#[packet(packet_id = "commands", state = "play")]
pub struct CommandsPacket {
    pub graph: LengthPrefixedVec<CommandNode>,
    pub root_idx: VarInt,
}

impl CommandsPacket {
    /// Creates a CommandsPacket from the provided command graph.
    pub fn new(graph: CommandGraph) -> Self {
        Self {
            graph: LengthPrefixedVec::new(graph.nodes),
            root_idx: VarInt::new(0),
        }
    }

    /// Creates a CommandsPacket using the globally registered command graph.
    ///
    /// This is the typical way to create this packet, as it includes all
    /// registered server commands for tab-completion and validation.
    pub fn from_global_graph() -> Self {
        Self::new(ferrumc_commands::infrastructure::get_graph())
    }
}

impl Default for CommandsPacket {
    fn default() -> Self {
        Self::from_global_graph()
    }
}
