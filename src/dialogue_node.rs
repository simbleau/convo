use crate::dialogue_link::*;

// A conversation node

#[derive(Debug)]
pub struct Node {
    /// The key of this node; should be unique
    key: String,

    /// The links to other nodes
    links: Vec<Link>,

    /// The description of the node
    description: String,
}

// Methods for a Node
impl Node {
    // Construct a node
    pub fn new(key: String, links: Vec<Link>, description: String) -> Node {
        Node {
            key,
            links,
            description,
        }
    }

    // Immutable access to node's key
    pub fn key(&self) -> &String {
        &self.key
    }

    // Mutable access to node's key
    pub fn key_mut(&mut self) -> &mut String {
        &mut self.key
    }

    // Immutable access to node's links
    pub fn links(&self) -> &Vec<Link> {
        &self.links
    }

    // Mutable access to node's links
    pub fn links_mut(&mut self) -> &mut Vec<Link> {
        &mut self.links
    }

    // Immutable access to node's description
    pub fn description(&self) -> &String {
        &self.description
    }

    // Mutable access to node's description
    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
}
