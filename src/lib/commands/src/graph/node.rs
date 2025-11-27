//! Command graph nodes.

use std::fmt;

use enum_ordinalize::Ordinalize;
use ferrumc_macros::NetEncode;
use ferrumc_protocol::codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};

use crate::arg::primitive::{PrimitiveArgumentFlags, PrimitiveArgumentType};

/// The type of command node.
#[derive(Clone, Debug, PartialEq, Ordinalize)]
pub enum CommandNodeType {
    Root,
    Literal,
    Argument,
}

impl CommandNodeType {
    /// Gets the protocol ID (ordinal) of this type.
    pub fn id(&self) -> u8 {
        self.ordinal() as u8
    }
}

/// Flags related to command nodes.
#[derive(Clone, Debug, PartialEq)]
pub enum CommandNodeFlag {
    /// The node type.
    NodeType(CommandNodeType),

    /// The node is executable.
    Executable,

    /// The node has a redirect.
    HasRedirect,

    /// The node has a suggestion type ([`CommandNodeType::Argument`] only).
    HasSuggestionsType,
}

impl CommandNodeFlag {
    /// Gets the bitmask of this flag.
    pub const fn bitmask(&self) -> u8 {
        match self {
            CommandNodeFlag::NodeType(CommandNodeType::Root) => 0x00,
            CommandNodeFlag::NodeType(CommandNodeType::Literal) => 0x01,
            CommandNodeFlag::NodeType(CommandNodeType::Argument) => 0x02,
            CommandNodeFlag::Executable => 0x04,
            CommandNodeFlag::HasRedirect => 0x08,
            CommandNodeFlag::HasSuggestionsType => 0x10,
        }
    }
}

/// An instance of a command node in a command graph.
#[derive(Clone, NetEncode)]
pub struct CommandNode {
    /// The encoded [`CommandNodeFlag`] of this node.
    pub flags: u8,

    /// Node indices of this node's children.
    pub children: LengthPrefixedVec<VarInt>,

    /// Node index of the redirected node. Only [`Some`] if `flags` is [`CommandNodeFlag::HasRedirect`].
    pub redirect_node: Option<VarInt>,

    /// The name of this node. Only [`None`] for the root node.
    pub name: Option<String>,

    /// The [`PrimitiveArgumentType`] of this node. Only [`Some`] for argument nodes.
    pub parser_id: Option<PrimitiveArgumentType>,

    /// The [`PrimitiveArgumentFlags`] of this node. Only [`Some`] for argument nodes.
    pub properties: Option<PrimitiveArgumentFlags>,

    /// The type of suggestions used for this node. Only [`Some`] for argument nodes.
    pub suggestions_type: Option<String>,
}

// We want to display the actual flags and not the encoded value
impl fmt::Debug for CommandNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node_type = match self.flags & 0x03 {
            0 => CommandNodeType::Root,
            1 => CommandNodeType::Literal,
            2 => CommandNodeType::Argument,
            _ => panic!("Invalid node type"),
        };

        let executable = self.flags & 0x04 != 0;
        let has_redirect = self.flags & 0x08 != 0;
        let has_suggestions_type = self.flags & 0x10 != 0;

        f.debug_struct("CommandNode")
            .field("node_type", &node_type)
            .field("executable", &executable)
            .field("has_redirect", &has_redirect)
            .field("has_suggestions_type", &has_suggestions_type)
            .field("flags", &self.flags)
            .field("children", &self.children)
            .field("redirect_node", &self.redirect_node)
            .field("name", &self.name)
            .field("parser_id", &self.parser_id)
            .field("properties", &self.properties)
            .field("suggestions_type", &self.suggestions_type)
            .finish()
    }
}

impl CommandNode {
    /// Gets the [`CommandNodeType`] from the flags.
    pub fn node_type(&self) -> CommandNodeType {
        match self.flags & 0x03 {
            1 => CommandNodeType::Literal,
            2 => CommandNodeType::Argument,
            _ => CommandNodeType::Root,
        }
    }

    /// Whether this node is executable.
    pub fn is_executable(&self) -> bool {
        self.flags & 0x04 != 0
    }

    /// Whether this node has a redirect.
    pub fn has_redirect(&self) -> bool {
        self.flags & 0x08 != 0
    }

    /// Whether this node has a suggestion type.
    pub fn has_suggestions_type(&self) -> bool {
        self.flags & 0x10 != 0
    }
}
