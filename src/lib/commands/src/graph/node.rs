use std::{collections::HashMap, fmt};

use enum_ordinalize::Ordinalize;

#[derive(Clone, Debug, PartialEq)]
pub enum CommandNodeType {
    Root,
    Literal,
    Argument,
}

impl CommandNodeType {
    pub const fn id(&self) -> u8 {
        match self {
            Self::Root => 0,
            Self::Literal => 1,
            Self::Argument => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandNodeFlag {
    NodeType(CommandNodeType),
    Executable,
    HasRedirect,
    HasSuggestionsType
}

impl CommandNodeFlag {
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

#[derive(Clone, Debug, PartialEq)]
pub enum CommandNodeProperties {
    IntRange { min: i32, max: i32 },
    FloatRange { min: f32, max: f32 },
    String { max_length: u32 },
    Other(HashMap<String, String>),
}

#[derive(Clone, PartialEq)]
pub struct CommandNode {
    pub flags: u8,
    pub children: Vec<u32>,
    pub redirect_node: Option<u32>,
    pub name: Option<String>,
    pub parser_id: Option<u32>,
    pub properties: Option<CommandNodeProperties>,
    pub suggestions_type: Option<String>
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
    pub fn node_type(&self) -> CommandNodeType {
        match self.flags & 0x03 {
            0 => CommandNodeType::Root,
            1 => CommandNodeType::Literal,
            2 => CommandNodeType::Argument,
            _ => panic!("Invalid node type"),
        }
    }

    pub fn is_executable(&self) -> bool {
        self.flags & 0x04 != 0
    }

    pub fn has_redirect(&self) -> bool {
        self.flags & 0x08 != 0
    }

    pub fn has_suggestions_type(&self) -> bool {
        self.flags & 0x10 != 0
    }
}

#[derive(Clone, Debug, PartialEq, Ordinalize)]
pub enum MinecraftCommandParser {
    Bool,
    Float,
    Double,
    Int,
    Long,
    String,
    Entity,
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    Component,
    Style,
    Message,
    Nbt,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operator,
    Particle,
    Angle,
    Rotation,
    ScoreboardDisplaySlot,
    ScoreHolder,
    UpTo3Axes,
    Team,
    ItemSlot,
    ResourceLocation,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    Dimension,
    GameMode,
    Time,
    ResourceOrTag,
    ResourceOrTagKey,
    Resource,
    ResourceKey,
    TemplateMirror,
    TemplateRotation,
    Heightmap,
    UUID
}

