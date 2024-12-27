use std::sync::Arc;
use std::collections::HashMap;

use node::{CommandNode, CommandNodeFlag, CommandNodeType};

use crate::Command;

pub mod node;

#[derive(Clone, Debug, PartialEq)]
pub struct CommandGraph {
    pub root_node: CommandNode,
    pub nodes: Vec<CommandNode>,
    pub node_to_indices: HashMap<String, u32>,
}

impl Default for CommandGraph {
    fn default() -> Self {
        let root_node = CommandNode {
            flags: CommandNodeFlag::NodeType(CommandNodeType::Root).bitmask(),
            children: Vec::new(),
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
            let node_type = if is_last {
                CommandNodeType::Literal
            } else {
                CommandNodeType::Literal
            };

            if let Some(&child_index) = self.node_to_indices.get(part) {
                current_node_index = child_index;
            } else {
                let mut node = CommandNode {
                    flags: CommandNodeFlag::NodeType(node_type).bitmask(),
                    children: Vec::new(),
                    redirect_node: None,
                    name: Some(part.to_string()),
                    parser_id: None,
                    properties: None,
                    suggestions_type: None,
                };

                if is_last && !command.args.is_empty() && command.args.first().is_some_and(|arg| !arg.required) {
                    node.flags |= CommandNodeFlag::Executable.bitmask();
                }

                let node_index = self.nodes.len() as u32;
                self.nodes.push(node);
                self.node_to_indices.insert(part.to_string(), node_index);
                self.root_node.children.push(node_index);

                self.nodes[current_node_index as usize].children.push(node_index);
                current_node_index = node_index;
            }
        }

        for arg in &command.args {
            let mut arg_node = CommandNode {
                flags: CommandNodeFlag::NodeType(CommandNodeType::Argument).bitmask(),
                children: Vec::new(),
                redirect_node: None,
                name: Some(arg.name.clone()),
                parser_id: None,
                properties: None,
                suggestions_type: None,
            };

            if arg.required {
                arg_node.flags |= CommandNodeFlag::Executable.bitmask();
            }

            let arg_node_index = self.nodes.len() as u32;
            self.nodes.push(arg_node);
            self.nodes[current_node_index as usize].children.push(arg_node_index);
        }
    }
}
