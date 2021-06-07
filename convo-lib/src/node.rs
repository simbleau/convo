use crate::link::Link;

// A conversation node

#[derive(Debug)]
pub struct Node {
    /// The key of this node; must be unique
    key: String,

    /// The dialogue of the node
    dialogue: String,

    /// The links to other nodes
    links: Vec<Link>,
}

// Methods for a Node
impl Node {
    // Construct a node
    pub fn new<T>(key: String, dialogue: T) -> Node
    where
        T: Into<String>,
    {
        Node {
            key,
            dialogue: dialogue.into(),
            links: vec![],
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

    // Immutable access to node's dialogue
    pub fn dialogue(&self) -> &String {
        &self.dialogue
    }

    // Mutable access to node's dialogue
    pub fn dialogue_mut(&mut self) -> &mut String {
        &mut self.dialogue
    }
}
