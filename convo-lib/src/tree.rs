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

    // Immutable access to root node
    pub fn root_node(&self) -> Option<&Node> {
        self.nodes.get(self.root.as_ref()?)
    }

    // Sets the root node to a new node defined by a key
    // Also sets current to root node if current is None
    pub fn set_root(&mut self, node_key: &str) -> Result<(), TreeError> {
        // Check existence
        if !self.nodes.contains_key(node_key) {
            return Err(TreeError::NodeDNE(node_key.to_owned()));
        }

        self.root = Some(node_key.to_owned());
        if self.current.is_none() {
            self.current = Some(node_key.to_owned());
        }
        Ok(())
    }

    // Sets the root node to a new node defined by a key
    pub unsafe fn set_root_unchecked(&mut self, node_key: &str) {
        self.root = Some(node_key.to_owned());
    }

    // Immutable access to current
    pub fn current(&self) -> Option<&String> {
        self.current.as_ref()
    }

    // Immutable access to current node
    pub fn current_node(&self) -> Option<&Node> {
        self.nodes.get(self.current.as_ref()?)
    }

    // Sets the current node to a new node defined by a key
    pub fn set_current(&mut self, node_key: &str) -> Result<(), TreeError> {
        // Check existence
        if !self.nodes.contains_key(node_key) {
            return Err(TreeError::NodeDNE(node_key.to_owned()));
        }

        self.current = Some(node_key.to_owned());
        Ok(())
    }

    // Sets the current node to a new node defined by a key
    pub unsafe fn set_current_unchecked(&mut self, node_key: &str) {
        self.current = Some(node_key.to_owned());
    }

    // Rewind the current node to root with safety
    pub fn rewind(&mut self) -> Result<(), TreeError> {
        if self.root.is_none() {
            return Err(TreeError::NodeDNE(String::from("Tree has no root node")));
        }

        self.current = self.root.clone();
        Ok(())
    }

    // Reset the current node to root
    pub unsafe fn rewind_unchecked(&mut self) {
        self.current = self.root.clone();
    }
}

#[cfg(test)]
#[test]
fn test_try_from() {
    let bad_source = "not valid source";
    assert!(CTree::try_from(bad_source).is_err());

    let mut good_file = std::fs::File::open("../examples/dialogue_files/ex_min.ctree.yml").unwrap();
    let mut good_source = String::new();
    std::io::Read::read_to_string(&mut good_file, &mut good_source).unwrap();

    assert!(CTree::try_from(&good_source).is_ok());
}
