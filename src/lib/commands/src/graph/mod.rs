//! The command graph for server or player command management.
//!
//! This module defines a graph structure where each node represents
//! a command or a command argument. It allows registering commands,
//! traversing them, and resolving input strings to commands.

use std::collections::HashMap;
use std::sync::Arc;

use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use node::{CommandNode, CommandNodeFlag, CommandNodeType};

use crate::Command;

pub mod node;

/// The command graph of the server or a player.
///
/// Holds references to all command nodes and their indices. Only
/// one instance exists on the server, shared across all players.
/// The graph is later sent to clients on join.
#[derive(Clone, Debug)]
pub struct CommandGraph {
    /// The root node of the command graph.
    pub root_node: CommandNode,

    /// All nodes in the graph, including the root and all children.
    pub nodes: Vec<CommandNode>,

    /// Mapping of node names to their indices in `nodes`.
    pub node_to_indices: HashMap<String, u32>,
}

impl Default for CommandGraph {
    /// Creates a default command graph with only the root node.
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
    /// Adds a command to the graph.
    ///
    /// Splits the command name into parts and adds each as a literal node.
    /// Then adds nodes for each argument of the command. Marks nodes as
    /// executable if appropriate.
    ///
    /// # Arguments
    /// * `command` - The command to add.
    pub fn push(&mut self, command: Arc<Command>) {
        let mut current_node_idx = 0;

        // Add literal nodes for command name
        for (idx, part) in command.name.split_whitespace().enumerate() {
            let is_last = idx == command.name.split_whitespace().count() - 1;

            let mut node = CommandNode {
                flags: CommandNodeFlag::NodeType(CommandNodeType::Literal).bitmask(),
                children: LengthPrefixedVec::new(Vec::new()),
                redirect_node: None,
                name: Some(part.to_string()),
                parser_id: None,
                properties: None,
                suggestions_type: None,
            };

            if is_last
                && (command.args.is_empty()
                || command.args.first().is_some_and(|arg| !arg.required))
            {
                node.flags |= CommandNodeFlag::Executable.bitmask();
            }

            let node_idx = self.nodes.len() as u32;
            self.nodes.push(node);
            self.node_to_indices.insert(part.to_string(), node_idx);

            if idx == 0 {
                self.nodes[0].children.push(VarInt::new(node_idx as i32));
            } else {
                let parent_node = self.nodes.get_mut(current_node_idx as usize).unwrap();
                parent_node.children.push(VarInt::new(node_idx as i32));
            }

            current_node_idx = node_idx;
        }

        let mut prev_node_idx = current_node_idx;

        // Add argument nodes
        for (idx, arg) in command.args.iter().enumerate() {
            let primitive = arg.primitive.clone();
            let is_last = idx == command.args.len() - 1;

            let mut arg_node = CommandNode {
                flags: CommandNodeFlag::NodeType(CommandNodeType::Argument).bitmask()
                    | CommandNodeFlag::HasSuggestionsType.bitmask(),
                children: LengthPrefixedVec::new(Vec::new()),
                redirect_node: None,
                name: Some(arg.name.clone()),
                parser_id: Some(primitive.argument_type),
                properties: primitive.flags,
                suggestions_type: Some("ask_server".to_string()),
            };

            if is_last {
                arg_node.flags |= CommandNodeFlag::Executable.bitmask();
            }

            let arg_node_idx = self.nodes.len() as u32;
            self.nodes.push(arg_node);

            self.nodes[prev_node_idx as usize]
                .children
                .push(VarInt::new(arg_node_idx as i32));

            prev_node_idx = arg_node_idx;
        }
    }

    /// Traverses the command graph, calling the given function on each node.
    ///
    /// # Arguments
    /// * `f` - Closure that receives: `(&CommandNode, node_idx, depth, parent_idx)`.
    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(&CommandNode, u32, usize, Option<u32>),
    {
        self.traverse_node(0, 0, None, &mut f);
    }

    fn traverse_node<F>(&self, node_idx: u32, depth: usize, parent: Option<u32>, f: &mut F)
    where
        F: FnMut(&CommandNode, u32, usize, Option<u32>),
    {
        let current_node = &self.nodes[node_idx as usize];
        f(current_node, node_idx, depth, parent);

        for child_idx in current_node.children.data.iter() {
            self.traverse_node(child_idx.0 as u32, depth + 1, Some(node_idx), f);
        }
    }

    /// Finds all matching commands for a given input string.
    ///
    /// Returns a vector of `(node_idx, remaining_input)`.
    ///
    /// # Arguments
    /// * `input` - The input string to match.
    pub fn find_command<'a>(&'a self, input: &'a str) -> Vec<(u32, &'a str)> {
        let mut matches = Vec::new();
        let input = input.trim();
        self.find_command_recursive(0, input, &mut matches);
        matches
    }

    fn find_command_recursive<'a>(
        &'a self,
        node_idx: u32,
        remaining_input: &'a str,
        matches: &mut Vec<(u32, &'a str)>,
    ) {
        let current_node = &self.nodes[node_idx as usize];
        let input_words: Vec<&str> = remaining_input.split_whitespace().collect();

        if remaining_input.is_empty() && current_node.is_executable() {
            matches.push((node_idx, remaining_input));
            return;
        }

        if remaining_input.is_empty() {
            return;
        }

        match current_node.node_type() {
            CommandNodeType::Root => {
                for child_idx in current_node.children.data.iter() {
                    self.find_command_recursive(child_idx.0 as u32, remaining_input, matches);
                }
            }
            CommandNodeType::Literal => {
                if let Some(name) = &current_node.name {
                    if !input_words.is_empty() && input_words[0] == name {
                        let remaining = if input_words.len() > 1 {
                            remaining_input[name.len()..].trim_start()
                        } else {
                            ""
                        };

                        if remaining.is_empty() && current_node.is_executable() {
                            matches.push((node_idx, remaining));
                        }

                        for child_idx in current_node.children.data.iter() {
                            self.find_command_recursive(child_idx.0 as u32, remaining, matches);
                        }
                    }
                }
            }
            CommandNodeType::Argument => {
                if !input_words.is_empty() {
                    let remaining = if input_words.len() > 1 {
                        remaining_input[input_words[0].len()..].trim_start()
                    } else {
                        ""
                    };

                    matches.push((node_idx, remaining));

                    for child_idx in current_node.children.data.iter() {
                        self.find_command_recursive(child_idx.0 as u32, remaining, matches);
                    }
                }
            }
        }
    }

    fn collect_command_parts(&self, node_idx: u32, parts: &mut Vec<String>) {
        let node = &self.nodes[node_idx as usize];

        if let Some(name) = &node.name {
            if node.node_type() == CommandNodeType::Literal {
                parts.push(name.clone());
            }
        }

        for (parent_idx, parent_node) in self.nodes.iter().enumerate() {
            if parent_node
                .children
                .data
                .iter()
                .any(|child| child.0 as u32 == node_idx)
            {
                self.collect_command_parts(parent_idx as u32, parts);
                break;
            }
        }
    }

    /// Returns the full command name for a given node index.
    pub fn get_command_name(&self, node_idx: u32) -> String {
        let mut parts = Vec::new();
        self.collect_command_parts(node_idx, &mut parts);
        parts.reverse();
        parts.join(" ")
    }

    /// Attempts to find a command by input and returns its name.
    ///
    /// # Arguments
    /// * `input` - The input string to resolve.
    ///
    /// # Returns
    /// `Some(String)` if a matching command is found, otherwise `None`.
    pub fn find_command_by_input(&self, input: &str) -> Option<String> {
        let matches = self.find_command(input);
        matches
            .first()
            .map(|(node_idx, _remaining)| self.get_command_name(*node_idx))
    }
}
