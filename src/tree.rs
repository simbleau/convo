use indexmap::IndexMap;
use std::path::Path;

use crate::{
    error::{ExportError, ImportError, TreeError},
    exporter,
    node::Node,
};

/// A [`Tree`] is the parent container for a conversation tree. It is a walkable structure which follows the form of a human conversation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tree {
    /// The nodes in this conversation tree. Each [`Node`] is uniquely indexable by its [`Node#key`][`Node#structfield.key`].
    pub nodes: IndexMap<String, Node>,

    /// The key of the root node. Can be [`None`]. If it is [`Some`], it is guaranteed to index an existing [`Node`] in [`Tree#nodes`][`Tree#structfield.nodes`].
    root_key: Option<String>,

    /// The key of the current node. Can be [`None`]. If it is [`Some`], it is guaranteed to index an existing [`Node`] in [`Tree#nodes`][`Tree#structfield.nodes`].
    current_key: Option<String>,
}

impl Default for Tree {
    fn default() -> Self {
        Tree::new()
    }
}

impl Tree {
    /// Returns a [`Tree`] with no nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::Tree;
    /// let tree = Tree::new();
    /// ```
    pub fn new() -> Self {
        Tree {
            nodes: IndexMap::<String, Node>::new(),
            root_key: None,
            current_key: None,
        }
    }

    /// Try to returns a [`Tree`] which is generated from parsing a string slice.
    ///
    /// # Arguments
    ///
    /// * `source` - A string slice that holds valid YAML data to parse from.
    /// See also: [example dialogue files](https://github.com/simbleau/convo/tree/main/examples/dialogue_files).
    ///
    /// # Errors
    ///
    /// * A [`ImportError`] will be returned if the source is not valid YAML data or if the tree is not considered legal when parsing.
    /// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::Tree;
    /// let source = r#"---
    /// root: start
    /// nodes:
    ///   start:
    ///     dialogue: I am a recursive node.
    ///     links:
    ///       - start: Recurse!"#;
    /// let tree = Tree::try_from(source).unwrap();
    /// ```
    pub fn try_from(source: &str) -> Result<Self, ImportError> {
        crate::importer::source_to_tree(source)
    }

    /// Try to export a [`Tree`] to a file. The preferred file extension is `*.convo.yml`.
    ///
    /// # Errors
    ///
    /// * An [`ExportError`] will be returned if the file is unable to be saved or the tree is not considered legal to export.
    /// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
    /// let root_key = "root";
    /// let root_node = Node::new(root_key, "The only node.");
    /// tree.nodes.insert(root_key.to_owned(), root_node);
    /// tree.set_root_key(root_key).unwrap();
    /// assert!(tree.try_export("examples/dialogue_files/export.convo.yml").is_ok());
    /// ```
    pub fn try_export<P>(&self, path: P) -> Result<(), ExportError>
    where
        P: AsRef<Path>,
    {
        exporter::export(self, path)
    }

    /// Returns an [`Option`] which references a copy of the root [`Node#key`][`Node#structfield.key`].
    /// This method will return [`None`] if the tree has no root set.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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

    /// Try to set the root node key for a [`Tree`]. If [`Tree#current`][`Tree#structfield.current`] is [`None`], this will automatically be dually initialized to the root key. If you want to set the root node without any [validation checks](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules), try [`set_root_key_unchecked`][`Tree#method.set_root_key_unchecked`].
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`Tree#nodes`][`Tree#structfield.nodes`].
    ///
    /// # Errors
    ///
    /// * A [`TreeError`] will be returned if the node does not exist in the node map.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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

    /// Set the root node key for a [`Tree`]. Unlike [`set_root_key`][`Tree#method.set_root_key`], this method will **not** incur side effects to [`Tree#current`][`Tree#structfield.current`] in any way.
    ///
    /// # Safety
    ///
    /// This function doesn't perform [validation checks](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`Tree#nodes`][`Tree#structfield.nodes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
    /// let root_og = Node::new("x", "Some node.");
    /// let root_copy = root_og.clone();
    /// tree.nodes.insert("x".to_owned(), root_copy);
    /// tree.set_current_key("x").unwrap();
    /// assert_eq!(&root_og, tree.current_node().unwrap());
    /// ```
    pub fn current_node(&self) -> Option<&Node> {
        self.nodes.get(self.current_key.as_ref()?)
    }

    /// Try to set the current node key for a [`Tree`]. If you want to set the current node without any [validation checks](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules), try [`set_current_key_unchecked`][`Tree#method.set_current_key_unchecked`].
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`Tree#nodes`][`Tree#structfield.nodes`].
    ///
    /// # Errors
    ///
    /// * A [`TreeError`] will be returned if the node does not exist in the node map.
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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

    /// Set the current node key for a [`Tree`].
    ///
    /// # Safety
    ///
    /// This function doesn't perform [validation checks](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
    ///
    /// # Arguments
    ///
    /// * `node_key` - A string slice that holds a unique identifier which indexes a [`Node`] in the [`Tree#nodes`][`Tree#structfield.nodes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
    /// let current_node = Node::new("x", "Some node.");
    /// tree.nodes.insert("x".to_owned(), current_node);
    /// unsafe { tree.set_current_key_unchecked("x"); }
    /// ```
    pub unsafe fn set_current_key_unchecked(&mut self, node_key: &str) {
        self.current_key = Some(node_key.to_owned());
    }

    /// Try to rewind the current node key for a [`Tree`] back to the root key by cloning the root key. If you want to rewind the current node without any [validation checks](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules), try [`rewind_unchecked`][`Tree#method.rewind_unchecked`].
    ///
    /// # Errors
    ///
    /// * A [`TreeError`] will be returned if the root node is [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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

    /// Rewind the current node key for a [`Tree`] back to the root key by cloning the root key.
    ///
    /// # Safety
    ///
    /// This function doesn't perform [validation checks](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
    ///
    /// # Examples
    ///
    /// ```
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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
    /// use convo::{Tree, Node};
    /// let mut tree = Tree::new();
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
    assert!(Tree::try_from(bad_source).is_err());

    let mut good_file = std::fs::File::open("examples/dialogue_files/ex_1.convo.yml").unwrap();
    let mut good_source = String::new();
    std::io::Read::read_to_string(&mut good_file, &mut good_source).unwrap();

    assert!(Tree::try_from(&good_source).is_ok());
}

#[test]
fn test_try_export() {
    let mut tree = Tree::new();

    // Should fail because validation checks fail
    assert!(tree
        .try_export("examples/dialogue_files/export.convo.yml")
        .is_err());

    // Qualify the tree
    let root_node = Node::new("root", "The only node.");
    tree.nodes.insert("root".to_owned(), root_node);
    tree.set_root_key("root").unwrap();

    // Should pass because tree is valid
    assert!(tree
        .try_export("examples/dialogue_files/export.convo.yml")
        .is_ok());
}

#[test]
fn test_root_key() {
    let mut tree = Tree::new();

    // Should be none because no root key has been set yet
    assert!(tree.root_key().is_none());

    unsafe { tree.set_root_key_unchecked("root") }

    // Should be Some
    assert_eq!("root", tree.root_key().unwrap());
}

#[test]
fn test_root_node() {
    let mut tree = Tree::new();

    // Should be none because no root key has been set yet
    assert!(tree.root_node().is_none());

    // Should be None still as the node does not exist in the map
    unsafe { tree.set_root_key_unchecked("root") }
    assert!(tree.root_node().is_none());

    // After insertion, it should exist
    let root_node = Node::new("root", "A node.");
    tree.nodes.insert("root".to_owned(), root_node);
    assert!(tree.root_node().is_some());
}

#[test]
fn test_set_root_key() {
    let mut tree = Tree::new();

    // Should fail because node does not exist in the map yet
    assert!(tree.set_root_key("root").is_err());

    // Should pass because node exists
    let root_node = Node::new("root", "Anode.");
    tree.nodes.insert("root".to_owned(), root_node);
    assert!(tree.set_root_key("root").is_ok());

    // Ensure root key was set
    assert_eq!("root", tree.root_key().unwrap());
}

#[test]
fn test_set_root_key_unchecked() {
    let mut tree = Tree::new();
    unsafe { tree.set_root_key_unchecked("root") }

    // Ensure root key was set
    assert_eq!("root", tree.root_key().unwrap());
}

#[test]
fn test_current_key() {
    let mut tree = Tree::new();

    // Should be none because no current key has been set yet
    assert!(tree.current_key().is_none());

    unsafe { tree.set_current_key_unchecked("current") }

    // Should be Some
    assert_eq!("current", tree.current_key().unwrap());
}

#[test]
fn test_current_node() {
    let mut tree = Tree::new();

    // Should be none because no current key has been set yet
    assert!(tree.current_node().is_none());

    // Should be None still as the node does not exist in the map
    unsafe { tree.set_current_key_unchecked("current") }
    assert!(tree.current_node().is_none());

    // After insertion, it should exist
    let current_node = Node::new("current", "A node.");
    tree.nodes.insert("current".to_owned(), current_node);
    assert!(tree.current_node().is_some());
}

#[test]
fn test_set_current_key() {
    let mut tree = Tree::new();

    // Should fail because node does not exist in the map yet
    assert!(tree.set_current_key("current").is_err());

    // Should pass because node exists
    let current_node = Node::new("current", "A node.");
    tree.nodes.insert("current".to_owned(), current_node);
    assert!(tree.set_current_key("current").is_ok());

    // Ensure current key was set
    assert_eq!("current", tree.current_key().unwrap());
}

#[test]
fn test_set_current_key_unchecked() {
    let mut tree = Tree::new();
    unsafe { tree.set_current_key_unchecked("current") }

    // Ensure current key was set
    assert_eq!("current", tree.current_key().unwrap());
}

#[test]
fn test_rewind() {
    // Set up tree with a root and additional node
    let mut tree = Tree::new();
    let root_node = Node::new("root", "The root node.");
    let current_node = Node::new("current", "A node.");
    tree.nodes.insert("root".to_owned(), root_node);
    tree.nodes.insert("current".to_owned(), current_node);

    tree.set_current_key("current").unwrap();

    // Should error as root is not set
    assert!(tree.rewind().is_err());

    // Test rewind
    assert_eq!("current", tree.current_key().unwrap());
    tree.set_root_key("root").unwrap();
    tree.rewind().unwrap();
    assert_eq!("root", tree.current_key().unwrap());
}

#[test]
fn test_rewind_unchecked() {
    // Set up tree with a root and additional node
    let mut tree = Tree::new();
    let root_node = Node::new("root", "The root node.");
    let current_node = Node::new("current", "A node.");
    tree.nodes.insert("root".to_owned(), root_node);
    tree.nodes.insert("current".to_owned(), current_node);

    tree.set_current_key("current").unwrap();

    // Test rewind
    assert_eq!("current", tree.current_key().unwrap());
    unsafe { tree.rewind_unchecked() }
    assert!(tree.root_node().is_none());
}

#[test]
fn test_reset() {
    // Set up tree with a root and additional node
    let mut tree = Tree::new();
    let root_node = Node::new("root", "The root node.");
    tree.nodes.insert("root".to_owned(), root_node);
    tree.set_root_key("root").unwrap();

    tree.reset();

    // Test reset
    assert_eq!(0, tree.nodes.len());
    assert!(tree.root_node().is_none());
    assert!(tree.current_node().is_none());
}
