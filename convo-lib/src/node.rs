use crate::link::Link;

// A conversation node

#[derive(Debug)]
pub struct Node {
    /// The key of this node; must be unique
    pub key: String,

    /// The dialogue of the node
    pub dialogue: String,

    /// The links to other nodes
    pub links: Vec<Link>,
}

// Methods for a Node
impl Node {
    // Construct a node
    pub fn new<T>(key: T, dialogue: T) -> Node
    where
        T: Into<String>,
    {
        Node {
            key: key.into(),
            dialogue: dialogue.into(),
            links: vec![],
        }
    }
}
