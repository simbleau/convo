use std::collections::HashMap;

use crate::node::Node;

#[derive(Debug)]
pub struct CTree {
    nodes: HashMap<String, Node>,
    root: Option<String>,
    current: Option<String>,
}

impl Default for CTree {
    // Provides a default empty CTree
    fn default() -> Self {
        CTree {
            nodes: HashMap::<String, Node>::new(),
            root: None,
            current: None,
        }
    }
}

impl CTree {
    // Construct a dialogue tree
    pub fn from(_source: &str, _root: &str) -> Result<CTree, &'static str> {
        todo!("Not yet implemented");
    }

    // Immutable access to root
    pub fn root(&self) -> Option<&String> {
        self.root.as_ref()
    }

    // Set the root
    pub fn set_root(&mut self, node: &Node) -> Result<&CTree, &'static str> {
        // Take ownership if necessary
        if self.root.is_some() {
            self.root.take().unwrap();
        }

        self.root = Some(node.key().clone());
        Ok(self)
    }

    // Reset the current node to root
    pub fn reset(&mut self) -> Result<&CTree, &'static str> {
        if self.root.is_none() {
            return Err("Root is none");
        }

        // Take ownership if necessary
        if self.current.is_some() {
            self.current.take().unwrap();
        }

        self.current = Some(self.root().unwrap().clone());
        Ok(self)
    }

    // Immutable access to node
    pub fn nodes(&self) -> &HashMap<String, Node> {
        &self.nodes
    }

    // Mutable access to node
    pub fn nodes_mut(&mut self) -> &mut HashMap<String, Node> {
        &mut self.nodes
    }
}
