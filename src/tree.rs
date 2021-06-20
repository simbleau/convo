use std::{collections::HashMap, path::Path};

use crate::{
    exporter::{self, ExportError},
    node::Node,
    parser::ParseError,
};

/// A [`TreeError`] is a category of errors returned by [`CTree`] methods which returns [`Result`]s.
#[derive(Debug)]
pub enum TreeError {
    /// An error caused when a [`CTree`] is missing a root [`Node`].
    /// See also: [`CTree#root`][`CTree#structfield.root].
    RootNotSet(),
    /// An error caused when a [`CTree`] is missing a current [`Node`].
    /// See also: [`CTree#current`][`CTree#structfield.current].
    CurrentNotSet(),
    /// An error caused when a [`CTree`] can not find a [`Node`].
    NodeDNE(String),
    /// An error caused when validating a family rules a [`CTree`] must obey.
    ///
    /// # Rules
    /// * [`Node`]s inserted must have unique keys.
    /// * [`CTree`]s must have a root node specified when parsing.
    /// * TODO: More
    Validation(String),
}

/// A [`CTree`] is the parent container for a conversation tree. It is a walkable structure which follows the form of a human conversation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CTree {
    /// The nodes in this conversation tree. Each [`Node`] is uniquely indexable by its [`Node#key`][`Node#structfield.key`].
    pub nodes: HashMap<String, Node>,

    /// The key of the root node. Can be [`None`]. If it is [`Some`], it is guaranteed to index an existing [`Node`] in [`CTree#nodes`][`CTree#structfield.nodes`].
    root_key: Option<String>,

    /// The key of the current node. Can be [`None`]. If it is [`Some`], it is guaranteed to index an existing [`Node`] in [`CTree#nodes`][`CTree#structfield.nodes`].
    current_key: Option<String>,
}

impl Default for CTree {
    fn default() -> Self {
        CTree::new()
    }
}

impl CTree {
    /// Returns a [`CTree`] with no nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::CTree;
    /// let tree = CTree::new();
    /// ```
    pub fn new() -> Self {
        CTree {
            nodes: HashMap::<String, Node>::new(),
            root_key: None,
            current_key: None,
        }
    }

    /// Try to returns a [`CTree`] which is generated from parsing a string slice.
    ///
    /// # Arguments
    ///
    /// * `source` - A string slice that holds valid YAML data to parse from.
    /// See also: [example dialogue files](https://github.com/simbleau/convo/tree/main/examples/dialogue_files).
    ///
    /// # Errors
    ///
    /// * A [`ParseError`] will be returned if the source is not valid YAML data or if the data breaks validation rules.
    /// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::CTree;
    /// let source = r#"
    /// ---
    /// root: start
    /// nodes:
    ///     start:
    ///         dialogue: I am a recursive node.
    ///         links:
    ///             - start: Recurse!
    /// "#;
    /// let tree = CTree::try_from(source).unwrap();
    /// ```
    pub fn try_from(source: &str) -> Result<Self, ParseError> {
        Ok(crate::parser::source_to_ctree(source)?)
    }

    /// Try to export a [`CTree`] to a file. The preferred file extension is `*.ctree.yml`.
    ///
    /// # Errors
    ///
    /// * An [`ExportError`] will be returned if the file is unable to be saved or the tree is not in a saveable state because it breaks validation rules.
    /// See also: [validation rules](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md#validation-rules).
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let root_key = "root";
    /// let root_node = Node::new(root_key, "The only node.");
    /// tree.nodes.insert(root_key.to_owned(), root_node);
    /// tree.set_root_key(root_key).unwrap();
    /// assert!(tree.try_export("example.ctree.yml").is_ok());
    /// ```
    pub fn try_export<P>(&self, path: P) -> Result<(), ExportError>
    where
        P: AsRef<Path>,
    {
        Ok(exporter::export(self, path)?)
    }

    /// Returns an [`Option`] which references a copy of the root [`Node#key`][`Node#structfield.key`].
    /// This method will return [`None`] if the tree has no root set.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// unsafe { tree.set_root_key_unchecked("root"); }
    /// assert_eq!("root", tree.root_key().unwrap());
    /// ```
    pub fn root_key(&self) -> Option<&String> {
        self.root_key.as_ref()
    }

    /// Returns an [`Option`] which references a copy of the root [`Node`][`Node`].
    /// This method will return [`None`] if the tree has no root set.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let root_og = Node::new("root", "The only node.");
    /// let root_copy = root_og.clone();
    /// tree.nodes.insert("root".to_owned(), root_copy);
    /// tree.set_root_key("root").unwrap();
    /// assert_eq!(&root_og, tree.root_node().unwrap());
    /// ```
    pub fn root_node(&self) -> Option<&Node> {
        self.nodes.get(self.root_key.as_ref()?)
    }

    // Sets the root node to a new node defined by a key
    // Also sets current to root node if current is None

    /// Try to set the root node key for a [`CTree`]. If [`CTree#current`][`CTree#structfield.current`] is [`None`], this will automatically be dually initialized to the root key. If you want to set the root node without any [validation checks](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md#validation-rules), try [`set_root_key_unchecked`][`CTree#method.set_root_key_unchecked`].
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`CTree#nodes`][`CTree#structfield.nodes`].
    ///
    /// # Errors
    ///
    /// * A [`TreeError`] will be returned if the node does not exist in the node map.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let root_node = Node::new("root", "The only node.");
    /// tree.nodes.insert("root".to_owned(), root_node);
    /// tree.set_root_key("root").unwrap();
    /// ```
    pub fn set_root_key(&mut self, node_key: &str) -> Result<(), TreeError> {
        // Check existence
        if !self.nodes.contains_key(node_key) {
            return Err(TreeError::NodeDNE(node_key.to_owned()));
        }

        self.root_key = Some(node_key.to_owned());
        if self.current_key.is_none() {
            self.current_key = Some(node_key.to_owned());
        }
        Ok(())
    }

    /// Set the root node key for a [`CTree`] without [validation checks](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md#validation-rules). Unlike [`set_root_key`][`CTree#method.set_root_key`], this method will **not** incur side effects to [`CTree#current`][`CTree#structfield.current`] in any way.
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`CTree#nodes`][`CTree#structfield.nodes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let root_node = Node::new("root", "The only node.");
    /// tree.nodes.insert("root".to_owned(), root_node);
    /// unsafe { tree.set_root_key_unchecked("root"); }
    /// ```
    pub unsafe fn set_root_key_unchecked(&mut self, node_key: &str) {
        self.root_key = Some(node_key.to_owned());
    }

    /// Returns an [`Option`] which references a copy of the current [`Node#key`][`Node#structfield.key`].
    /// This method will return [`None`] if the tree has no current set.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// unsafe { tree.set_current_key_unchecked("x"); }
    /// assert_eq!("x", tree.current_key().unwrap());
    /// ```
    pub fn current_key(&self) -> Option<&String> {
        self.current_key.as_ref()
    }

    /// Returns an [`Option`] which references a copy of the current [`Node`][`Node`].
    /// This method will return [`None`] if the tree has no current set.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let root_og = Node::new("x", "Some node.");
    /// let root_copy = root_og.clone();
    /// tree.nodes.insert("x".to_owned(), root_copy);
    /// tree.set_current_key("x").unwrap();
    /// assert_eq!(&root_og, tree.current_node().unwrap());
    /// ```
    pub fn current_node(&self) -> Option<&Node> {
        self.nodes.get(self.current_key.as_ref()?)
    }

    /// Try to set the current node key for a [`CTree`]. If you want to set the current node without any [validation checks](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md#validation-rules), try [`set_current_key_unchecked`][`CTree#method.set_current_key_unchecked`].
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`CTree#nodes`][`CTree#structfield.nodes`].
    ///
    /// # Errors
    ///
    /// * A [`TreeError`] will be returned if the node does not exist in the node map.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let current_node = Node::new("x", "Some node.");
    /// tree.nodes.insert("x".to_owned(), current_node);
    /// tree.set_current_key("x").unwrap();
    /// ```
    pub fn set_current_key(&mut self, node_key: &str) -> Result<(), TreeError> {
        // Check existence
        if !self.nodes.contains_key(node_key) {
            return Err(TreeError::NodeDNE(node_key.to_owned()));
        }

        self.current_key = Some(node_key.to_owned());
        Ok(())
    }

    /// Set the current node key for a [`CTree`] without [validation checks](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md#validation-rules).
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`CTree#nodes`][`CTree#structfield.nodes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let current_node = Node::new("x", "Some node.");
    /// tree.nodes.insert("x".to_owned(), current_node);
    /// unsafe { tree.set_current_key_unchecked("x"); }
    /// ```
    pub unsafe fn set_current_key_unchecked(&mut self, node_key: &str) {
        self.current_key = Some(node_key.to_owned());
    }

    /// Try to rewind the current node key for a [`CTree`] back to the root key by cloning the root key. If you want to rewind the current node without any [validation checks](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md#validation-rules), try [`rewind_unchecked`][`CTree#method.rewind_unchecked`].
    ///
    /// # Errors
    ///
    /// * A [`TreeError`] will be returned if the root node is [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let root_node = Node::new("root", "The root.");
    /// let current_node = Node::new("x", "Some node.");
    /// tree.nodes.insert("root".to_owned(), root_node);
    /// tree.nodes.insert("x".to_owned(), current_node);
    /// tree.set_root_key("root").unwrap();
    /// tree.set_current_key("x").unwrap();
    /// tree.rewind().unwrap();
    /// assert_eq!("root", tree.current_key().unwrap());
    /// ```
    pub fn rewind(&mut self) -> Result<(), TreeError> {
        if self.root_key.is_none() {
            return Err(TreeError::RootNotSet());
        }

        self.current_key = self.root_key.clone();
        Ok(())
    }

    /// Rewind the current node key for a [`CTree`] back to the root key by cloning the root key.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let current_node = Node::new("x", "Some node.");
    /// tree.nodes.insert("x".to_owned(), current_node);
    /// tree.set_current_key("x").unwrap();
    /// unsafe { tree.rewind_unchecked(); }
    /// assert!(tree.current_key().is_none()); // Because the root was `None`.
    /// ```
    pub unsafe fn rewind_unchecked(&mut self) {
        self.current_key = self.root_key.clone();
    }

    /// Clear the entire tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{CTree, Node};
    /// let mut tree = CTree::new();
    /// let root_node = Node::new("root", "The root.");
    /// tree.nodes.insert("root".to_owned(), root_node);
    /// tree.set_root_key("root").unwrap();
    /// tree.reset();
    /// assert_eq!(0, tree.nodes.len());
    /// assert!(tree.root_key().is_none());
    /// assert!(tree.current_key().is_none());
    /// ```
    pub fn reset(&mut self) {
        self.nodes.clear();
        self.root_key = None;
        self.current_key = None;
    }
}

#[cfg(test)]
#[test]
fn test_try_from() {
    let bad_source = "not valid source";
    assert!(CTree::try_from(bad_source).is_err());

    let mut good_file = std::fs::File::open("examples/dialogue_files/ex_1.ctree.yml").unwrap();
    let mut good_source = String::new();
    std::io::Read::read_to_string(&mut good_file, &mut good_source).unwrap();

    assert!(CTree::try_from(&good_source).is_ok());
}

#[test]
fn test_try_export() {
    let tree = crate::parser::parse("examples/dialogue_files/ex_1.ctree.yml").unwrap();
    let source = crate::exporter::ctree_to_source(&tree).unwrap();
    println!("{}", source);
}
