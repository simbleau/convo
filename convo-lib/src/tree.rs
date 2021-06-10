use std::collections::HashMap;

use crate::{node::Node, parser::ParseError};

#[derive(Debug)]
pub enum TreeError {
    NodeDNE(String),
    Validation(String),
}

#[derive(Debug)]
pub struct CTree {
    pub nodes: HashMap<String, Node>,
    root: Option<String>,
    current: Option<String>,
}

impl Default for CTree {
    fn default() -> Self {
        CTree::new()
    }
}

impl CTree {
    // Provides a default empty CTree
    pub fn new() -> Self {
        CTree {
            nodes: HashMap::<String, Node>::new(),
            root: None,
            current: None,
        }
    }

    // Construct a dialogue tree
    pub fn try_from(source: &str) -> Result<Self, ParseError> {
        Ok(crate::parser::source_to_ctree(source)?)
    }

    // Immutable access to root
    pub fn root(&self) -> Option<&String> {
        self.root.as_ref()
    }

    // Sets the root node to a new node defined by a key with node
    pub fn set_root(&mut self, node_key: &str) -> Result<(), TreeError> {
        // Check existence
        if !self.nodes.contains_key(node_key) {
            return Err(TreeError::NodeDNE(node_key.to_owned()));
        }

        self.root = Some(node_key.to_owned());
        Ok(())
    }

    // Sets the root node to a new node defined by a key
    pub unsafe fn set_root_unchecked(&mut self, node_key: &str) {
        self.root = Some(node_key.to_owned());
    }

    // Reset the current node to root with root checking
    pub fn reset(&mut self) -> Result<(), TreeError> {
        if self.root.is_none() {
            return Err(TreeError::NodeDNE(String::from("Tree has no root node")));
        }

        self.current = self.root.clone();
        Ok(())
    }

    // Reset the current node to root
    pub unsafe fn reset_unchecked(&mut self) {
        self.current = self.root.clone();
    }
}
