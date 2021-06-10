use std::collections::HashMap;

use crate::{node::Node, parser::ParseError};

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
    pub fn try_from(source: &str, root: &str) -> Result<Self, ParseError> {
        let mut tree = crate::parser::source_to_ctree(source)?;
        if !tree.nodes.contains_key(root) {
            return Err("TODO : Useful error message".into());
        }
        tree.root = Some(root.to_owned());
        Ok(tree)
    }

    // Immutable access to root
    pub fn root(&self) -> Option<&String> {
        self.root.as_ref()
    }

    pub fn set_root(&mut self, node_key: &str) -> Result<&mut Self, &'static str> {
        // Take ownership if necessary
        if self.root.is_some() {
            self.root.take();
        }

        // Check existence
        if !self.nodes.contains_key(node_key) {
            return Err("TODO : Useful error message");
        }

        self.root = Some(node_key.to_owned());
        Ok(self)
    }

    // Reset the current node to root
    pub fn reset(&mut self) -> Result<&mut Self, &'static str> {
        if self.root.is_none() {
            return Err("Root is none");
        }

        // Take ownership if necessary
        if self.current.is_some() {
            self.current.take();
        }

        self.current = Some(self.root().unwrap().clone());
        Ok(self)
    }
}
