//! The command graph.

use std::collections::HashMap;
use std::sync::Arc;

use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use node::{CommandNode, CommandNodeFlag, CommandNodeType};

use crate::Command;

pub mod node;

/// The command graph of the server or an individual player.
/// As of now, only one instance of this exists on the entire
/// server and it is shared between all players. The command
/// graph holds references to all command nodes and maps them
/// to their indices, and is later sent to the client on join.
#[derive(Clone, Debug)]
pub struct CommandGraph {
    /// The root node.
    pub root_node: CommandNode,

    /// The root node with all its child nodes.
    pub nodes: Vec<CommandNode>,

    /// A map of command node parts to indices.
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
    /// Adds the given `command` onto this command graph.
    pub fn push(&mut self, command: Arc<Command>) {
        let mut current_node_idx = 0;

        for (i, part) in command.name.split_whitespace().enumerate() {
            let is_last = i == command.name.split_whitespace().count() - 1;

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

            if i == 0 {
                self.nodes[0].children.push(VarInt::new(node_idx as i32));
            } else {
                let parent_node = self.nodes.get_mut(current_node_idx as usize).unwrap();
                parent_node.children.push(VarInt::new(node_idx as i32));
            }

            current_node_idx = node_idx;
        }

        let mut prev_node_idx = current_node_idx;

        for (i, arg) in command.args.iter().enumerate() {
            let primitive = arg.primitive.clone();
            let is_last = i == command.args.len() - 1;

            let mut arg_node = CommandNode {
                flags: CommandNodeFlag::NodeType(CommandNodeType::Argument).bitmask() | CommandNodeFlag::HasSuggestionsType.bitmask(),
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

    /// Traverses the command graph with a given function.
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

    /// Attempts to find the matches to a given `input` string and returns
    /// a vector of the node index and command name.
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

        // once the input is empty and the currently selected node is executable, we've found it.
        if remaining_input.is_empty() && current_node.is_executable() {
            matches.push((node_idx, remaining_input));
            return;
        }

        // once the input is empty but the currently selected node is not executable, we check the children.
        if remaining_input.is_empty() {
            return;
        }

        match current_node.node_type() {
            CommandNodeType::Root => {
                // the root node is the root of all evil.
                for child_idx in current_node.children.data.iter() {
                    self.find_command_recursive(child_idx.0 as u32, remaining_input, matches);
                }
            }
            CommandNodeType::Literal => {
                // for literal nodes, everything must match exactly.
                if let Some(name) = &current_node.name {
                    if !input_words.is_empty() && input_words[0] == name {
                        // we found a match, we continue with the remaining input.
                        let remaining = if input_words.len() > 1 {
                            remaining_input[name.len()..].trim_start()
                        } else {
                            ""
                        };

                        // once we found a node that is executable and the remaining input is empty, we've found something.
                        if remaining.is_empty() && current_node.is_executable() {
                            matches.push((node_idx, remaining));
                        }

                        // we continue checking the other children.
                        for child_idx in current_node.children.data.iter() {
                            self.find_command_recursive(child_idx.0 as u32, remaining, matches);
                        }
                    }
                }
            }
            CommandNodeType::Argument => {
                // for argument nodes, we consume one argument and then continue.
                if !input_words.is_empty() {
                    let remaining = if input_words.len() > 1 {
                        remaining_input[input_words[0].len()..].trim_start()
                    } else {
                        ""
                    };

                    // if this node is executable, we add it.
                    matches.push((node_idx, remaining));

                    // continue checking anyway.
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

        // find the parent
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

    /// Gets the name of a command based off the `node_idx`.
    pub fn get_command_name(&self, node_idx: u32) -> String {
        let mut parts = Vec::new();
        self.collect_command_parts(node_idx, &mut parts);
        parts.reverse(); // reverse since we want the command name in proper order
        parts.join(" ")
    }

    /// Attempts to find a command from the given `input` and returns the command name.
    pub fn find_command_by_input(&self, input: &str) -> Option<String> {
        let matches = self.find_command(input);

        matches
            .first()
            .map(|(node_idx, _remaining)| self.get_command_name(*node_idx))
    }
}
