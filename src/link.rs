use crate::node::Node;

/// A [`Link`] is a uni-directional path to a [`Node`] with descriptor [`dialogue`][`Link#structfield.dialogue`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Link {
    /// A key to the node being linked. This should be identical to an existing [`Node#key`][`Node#structfield.key`].
    pub to_key: String,

    /// The dialogue used to describe this link.
    pub dialogue: String,
}

impl Link {
    /// Returns a [`Link`] which maps to a [`Node`] with descriptor dialogue.
    ///
    /// # Arguments
    ///
    /// * `to_key` - A string type that holds an identical [`Node#key`][`Node#structfield.key`] to which this [`Link`] corresponds.
    /// * `dialogue` - A string type that holds associated descriptor dialogue.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::Link;
    /// let link = Link::new("end", "Goodbye!");
    /// ```
    pub fn new<T>(to_key: T, dialogue: T) -> Link
    where
        T: Into<String>,
    {
        Link {
            to_key: to_key.into(),
            dialogue: dialogue.into(),
        }
    }

    /// Link two [`Node`]s together by creating a [`Link`] with descriptor dialogue.
    ///
    /// # Arguments
    ///
    /// * `from` - A [`Node`] which will prompt the link
    /// * `to` - A [`Node`] which will be the target of the link
    /// * `dialogue` - A string type that holds associated dialogue
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Node, Link};
    /// let mut node1 = Node::new("root", "I am the root node!");
    /// let node2 = Node::new("end", "I am the last node!");
    /// Link::link(&mut node1, &node2, "I link start to end!");
    /// ```
    pub fn link<T>(from: &mut Node, to: &Node, dialogue: T)
    where
        T: Into<String>,
    {
        let link = Link {
            to_key: to.key.clone(),
            dialogue: dialogue.into(),
        };
        from.links.push(link);
    }
}
