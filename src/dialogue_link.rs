/// A mapping to a node.
#[derive(Debug)]
pub struct Link {
    // The key for the node this links to
    to: String,

    /// The description of the link
    description: String,
}

impl Link {
    // Construct a link
    pub fn from(to: String, description: String) -> Link {
        Link { to, description }
    }

    // Immutable access to node
    pub fn to(&self) -> &String {
        &self.to
    }

    // Mutable access to node
    pub fn to_mut(&mut self) -> &mut String {
        &mut self.to
    }

    // Immutable access to link's description
    pub fn description(&self) -> &str {
        &self.description
    }

    // Mutable access to link's description
    pub fn description_mut(&mut self) -> &mut str {
        &mut self.description
    }
}
