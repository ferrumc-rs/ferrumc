use std::sync::Arc;
use std::{collections::HashMap, io::Write};

use enum_ordinalize::Ordinalize;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use node::{CommandNode, CommandNodeFlag, CommandNodeType, MinecraftCommandParser};

use crate::infrastructure::get_graph;
use crate::Command;

pub mod node;

#[derive(Clone, Debug)]
pub struct CommandGraph {
    pub root_node: CommandNode,
    pub nodes: Vec<CommandNode>,
    pub node_to_indices: HashMap<String, u32>,
}

impl Default for CommandGraph {
    fn default() -> Self {
        let root_node = CommandNode {
            flags: CommandNodeFlag::NodeType(CommandNodeType::Root).bitmask(),
            children: LengthPrefixedVec::new(Vec::new()),
            redirect_node: None,
            name: None,
            parser_id: None,
            properties: None,
            suggestions_type: None,
        };

        Self {
            root_node: root_node.clone(),
            nodes: vec![root_node],
            node_to_indices: HashMap::new(),
        }
    }
}

impl CommandGraph {
    pub fn push(&mut self, command: Arc<Command>) {
        let mut current_node_index = 0;

        for (i, part) in command.name.split_whitespace().enumerate() {
            let is_last = i == command.name.split_whitespace().count() - 1;

            if let Some(&child_index) = self.node_to_indices.get(part) {
                current_node_index = child_index;
            } else {
                let mut node = CommandNode {
                    flags: CommandNodeFlag::NodeType(CommandNodeType::Literal).bitmask()
                        | CommandNodeFlag::Executable.bitmask(),
                    children: LengthPrefixedVec::new(Vec::new()),
                    redirect_node: None,
                    name: Some(part.to_string()),
                    parser_id: None,
                    properties: None,
                    suggestions_type: None,
                };

                if is_last
                    && !command.args.is_empty()
                    && command.args.first().is_some_and(|arg| !arg.required)
                {
                    node.flags |= CommandNodeFlag::Executable.bitmask();
                }

                let node_index = self.nodes.len() as u32;
                self.nodes.push(node);
                self.node_to_indices.insert(part.to_string(), node_index);
                let node_index_varint = VarInt::new(node_index as i32);

                self.root_node.children.push(node_index_varint.clone());

                let node = self.nodes.get_mut(current_node_index as usize).unwrap();
                node.children.push(node_index_varint);
                current_node_index = node_index;
            }
        }

        for arg in &command.args {
            let arg_node = CommandNode {
                flags: CommandNodeFlag::NodeType(CommandNodeType::Argument).bitmask(),
                children: LengthPrefixedVec::new(Vec::new()),
                redirect_node: None,
                name: Some(arg.name.clone()),
                parser_id: Some(VarInt::new(MinecraftCommandParser::String.ordinal() as i32)),
                properties: Some(node::CommandNodeProperties::String {
                    behavior: VarInt::new(2),
                }),
                suggestions_type: None,
            };

            let arg_node_index = self.nodes.len() as u32;
            self.nodes.push(arg_node);
            self.nodes[current_node_index as usize]
                .children
                .push(VarInt::new(arg_node_index as i32));
        }
    }

    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(&CommandNode, u32, usize, Option<u32>),
    {
        self.traverse_node(0, 0, None, &mut f);
    }

    fn traverse_node<F>(&self, node_index: u32, depth: usize, parent: Option<u32>, f: &mut F)
    where
        F: FnMut(&CommandNode, u32, usize, Option<u32>),
    {
        let current_node = &self.nodes[node_index as usize];

        f(current_node, node_index, depth, parent);

        for child_index in current_node.children.data.iter() {
            self.traverse_node(child_index.val as u32, depth + 1, Some(node_index), f);
        }
    }

    pub fn find_command<'a>(&'a self, input: &'a str) -> Vec<(u32, &'a str)> {
        let mut matches = Vec::new();
        let input = input.trim();

        self.find_command_recursive(0, input, &mut matches);
        matches
    }

    fn find_command_recursive<'a>(
        &'a self,
        node_index: u32,
        remaining_input: &'a str,
        matches: &mut Vec<(u32, &'a str)>,
    ) {
        let current_node = &self.nodes[node_index as usize];
        let input_words: Vec<&str> = remaining_input.split_whitespace().collect();

        // once the input is empty and the currently selected node is executable, we've found it.
        if remaining_input.is_empty() && current_node.is_executable() {
            matches.push((node_index, remaining_input));
            return;
        }

        // once the input is empty but the currently selected node is not executable, we check the children.
        if remaining_input.is_empty() {
            return;
        }

        match current_node.node_type() {
            CommandNodeType::Root => {
                // the root node is the root of all evil.
                for child_index in current_node.children.data.iter() {
                    self.find_command_recursive(child_index.val as u32, remaining_input, matches);
                }
            }
            CommandNodeType::Literal => {
                // for literal nodes, everything must match exactly.
                if let Some(name) = &current_node.name {
                    if !input_words.is_empty() && input_words[0] == name {
                        // we found a match, we continue with the remaining input.
                        let remaining = if input_words.len() > 1 {
                            &remaining_input[name.len()..].trim_start()
                        } else {
                            ""
                        };

                        // once we found a node that is executable and the remaining input is empty, we've found something.
                        if remaining.is_empty() && current_node.is_executable() {
                            matches.push((node_index, remaining));
                        }

                        // we continue checking the other children.
                        for child_index in current_node.children.data.iter() {
                            self.find_command_recursive(child_index.val as u32, remaining, matches);
                        }
                    }
                }
            }
            CommandNodeType::Argument => {
                // for argument nodes, we consume one argument and then continue.
                if !input_words.is_empty() {
                    let remaining = if input_words.len() > 1 {
                        &remaining_input[input_words[0].len()..].trim_start()
                    } else {
                        ""
                    };

                    // if this node is executable, we add it.
                    matches.push((node_index, remaining));

                    // continue checking anyway.
                    for child_index in current_node.children.data.iter() {
                        self.find_command_recursive(child_index.val as u32, remaining, matches);
                    }
                }
            }
        }
    }

    fn collect_command_parts(&self, node_index: u32, parts: &mut Vec<String>) {
        let node = &self.nodes[node_index as usize];

        if let Some(name) = &node.name {
            if node.node_type() == CommandNodeType::Literal {
                parts.push(name.clone());
            }
        }

        // find the parent
        for (parent_idx, parent_node) in self.nodes.iter().enumerate() {
            if parent_node
                .children
                .data
                .iter()
                .any(|child| child.val as u32 == node_index)
            {
                self.collect_command_parts(parent_idx as u32, parts);
                break;
            }
        }
    }

    pub fn get_command_name(&self, node_index: u32) -> String {
        let mut parts = Vec::new();
        self.collect_command_parts(node_index, &mut parts);
        parts.reverse(); // reverse since we want the command name in proper order
        parts.join(" ")
    }

    pub fn find_command_by_input(&self, input: &str) -> Option<String> {
        let matches = self.find_command(input);

        matches
            .first()
            .map(|(node_index, _remaining)| self.get_command_name(*node_index))
    }
}

#[derive(NetEncode, Debug)]
#[packet(packet_id = 0x11)]
pub struct CommandsPacket {
    pub graph: LengthPrefixedVec<CommandNode>,
    pub root_idx: VarInt,
}

impl CommandsPacket {
    pub fn new(graph: CommandGraph) -> Self {
        Self {
            graph: LengthPrefixedVec::new(graph.nodes),
            root_idx: VarInt::new(0),
        }
    }

    pub fn create() -> Self {
        Self::new(get_graph())
    }
}
