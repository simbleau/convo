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
    pub fn new(to: &Node, dialogue: String) -> Link {
        Link {
            to: to.key().clone(),
            dialogue,
        }
    }

    // Create the link from one node to the next
    pub fn link(from: &mut Node, to: &Node, dialogue: String) {
        let link = Link {
            to: to.key().clone(),
            dialogue,
        };
        from.links_mut().insert(0, link);
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
