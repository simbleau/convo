use crate::link::Link;

/// A [`Node`] is a node in a conversation tree. It canonically acts as a fork of decisions by wrapping prompting [`dialogue`][`Node#structfield.dialogue`] and a list of path options (called [`Link`]s).
#[derive(Debug, Clone)]
pub struct Node {
    /// The key of this node. Must be unique.
    pub key: String,

    /// The dialogue of this node.
    pub dialogue: String,

    /// A container of [`Link`]s, which connect to other [`Node`]s.
    pub links: Vec<Link>,
}

impl Node {
    /// Returns a [`Node`] with prompting dialogue. The structure returned contains no links.
    ///
    /// # Arguments
    ///
    /// * `key` - A string type that holds unique identifier for indexing.
    /// * `dialogue` - A string type that holds associated descriptor dialogue.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::Node;
    /// let node = Node::new("start", "How are you?");
    /// ```
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
