use crate::node::Node;

/// A mapping to a node.
#[derive(Debug, Clone)]
pub struct Link {
    // The key for the node this links to
    pub to: String,

    /// The dialogue of the link
    pub dialogue: String,
}

impl Link {
    // Construct a link
    pub fn new<T>(to_key: T, dialogue: T) -> Link
    where
        T: Into<String>,
    {
        Link {
            to: to_key.into(),
            dialogue: dialogue.into(),
        }
    }

    // Create the link from one node to the next
    pub fn link<T>(from: &mut Node, to: &Node, dialogue: T)
    where
        T: Into<String>,
    {
        let link = Link {
            to: to.key.clone(),
            dialogue: dialogue.into(),
        };
        from.links.push(link);
    }
}
