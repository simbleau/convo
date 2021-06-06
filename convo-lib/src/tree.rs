use std::collections::HashMap;

use crate::{link::Link, node::Node};

#[derive(Debug)]
pub struct CTree {
    nodes: HashMap<String, Node>,
    root: String,
    current: String,
}

impl CTree {
    // Example tree for testing
    pub fn example_tree() -> CTree {
        // Build nodes
        let mut node1 = Node::new("root".to_string(), vec![], "How are you?".to_string());
        let node2 = Node::new("n2".to_string(), vec![], "Don't be late!".to_string());

        // Link node1 -> node 2
        node1.links_mut().insert(
            0,
            Link::from(node2.key().clone(), "In a hurry!".to_string()),
        );

        // Declare nodes
        let mut nodes = HashMap::<String, Node>::new();
        nodes.insert(node1.key().clone(), node1);
        nodes.insert(node2.key().clone(), node2);

        CTree {
            nodes,
            root: "root".to_string(),
            current: "root".to_string(),
        }
    }

    // Construct a dialogue tree
    pub fn from(_source: &str, _root: &str) -> Result<CTree, &'static str> {
        todo!("Not yet implemented");
    }

    // Reset the current node to root
    pub fn reset(&mut self) -> &CTree {
        self.current = self.root.clone();
        self
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
