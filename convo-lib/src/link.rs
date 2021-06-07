use crate::node::Node;

/// A mapping to a node.
#[derive(Debug)]
pub struct Link {
    // The key for the node this links to
    to: String,

    /// The dialogue of the link
    dialogue: String,
}

impl Link {
    // Construct a link
    pub fn new<T>(to: &Node, dialogue: T) -> Link
    where
        T: Into<String>,
    {
        Link {
            to: to.key().clone(),
            dialogue: dialogue.into(),
        }
    }

    // Create the link from one node to the next
    pub fn link<T>(from: &mut Node, to: &Node, dialogue: T)
    where
        T: Into<String>,
    {
        let link = Link {
            to: to.key().clone(),
            dialogue: dialogue.into(),
        };
        from.links_mut().push(link);
    }

    // Immutable access to node
    pub fn to(&self) -> &String {
        &self.to
    }

    // Mutable access to node
    pub fn to_mut(&mut self) -> &mut String {
        &mut self.to
    }

    // Immutable access to link's dialogue
    pub fn dialogue(&self) -> &str {
        &self.dialogue
    }

    // Mutable access to link's dialogue
    pub fn dialogue_mut(&mut self) -> &mut str {
        &mut self.dialogue
    }
}
