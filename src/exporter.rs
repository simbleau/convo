//! A family of functions which export [`Tree`]s into YAML data.

use crate::{
    error::{ExportError, TreeError},
    link::Link,
    node::Node,
    tree::Tree,
};

use std::{fs::File, io::Write, path::Path};
use yaml_rust::{yaml, Yaml, YamlEmitter};

/// Try to save a [`Tree`] as a file.
///
/// # Arguments
///
/// * `tree` - A [`Tree`] that will be saved in a file.
///
/// # Errors
///
/// * An [`ExportError`] will be returned if the tree is not considered legal or incurs issues saving.
/// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
///
/// # Examples
///
/// ```
/// use convo::{importer, exporter};
/// let tree = importer::import("examples/dialogue_files/ex_min.convo.yml").unwrap();
/// // Make a copy of the file
/// exporter::export(&tree, "examples/dialogue_files/export.convo.yml").unwrap();
/// ```
pub fn export<P>(tree: &Tree, path: P) -> Result<(), ExportError>
where
    P: AsRef<Path>,
{
    let source = tree_to_source(tree)?;

    // Write file
    let mut file = File::create(path)?;
    file.write_all(source.as_bytes())?;

    Ok(())
}

/// Try to returns a [`String`] which is generated as YAML from a [`Tree`].
///
/// # Arguments
///
/// * `tree` - A [`Tree`] that will be returned as YAML data.
///
/// # Errors
///
/// * An [`ExportError`] will be returned if the tree is not considered legal to export.
/// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
///
/// # Examples
///
/// ```
/// use convo::{importer, exporter};
/// let source = r#"---
/// root: start
/// nodes:
///   start:
///     dialogue: I am a recursive node.
///     links:
///       - start: Recurse!"#;
/// let tree = importer::source_to_tree(source).unwrap();
/// let source2 = exporter::tree_to_source(&tree).unwrap();
/// assert_eq!(source, source2);
/// ```
pub fn tree_to_source(tree: &Tree) -> Result<String, ExportError> {
    let yaml = tree_to_yaml(&tree)?;

    // Convert to source text
    let mut writer = String::new();
    let mut emitter = YamlEmitter::new(&mut writer);
    emitter.compact(true);
    emitter.dump(&yaml)?;

    Ok(writer)
}

fn tree_to_yaml(tree: &Tree) -> Result<Yaml, TreeError> {
    // Check root key exists
    let root_key = tree.root_key().ok_or_else(|| TreeError::RootNotSet())?;

    // Check length of nodes
    if tree.nodes.len() == 0 {
        return Err(TreeError::Validation("Node map has a length of 0".into()));
    }

    // Build node map
    let mut node_map = yaml::Hash::new();
    for (key, node) in &tree.nodes {
        let yaml_key = Yaml::String(key.to_owned());
        let yaml_node = node_to_yaml(&node)?;
        node_map.insert(yaml_key, yaml_node);
    }

    // Build the document
    let mut yaml = yaml::Hash::new();
    yaml.insert(
        Yaml::String("root".to_string()),
        Yaml::String(root_key.to_owned()),
    );
    yaml.insert(Yaml::String("nodes".to_string()), Yaml::Hash(node_map));

    Ok(Yaml::Hash(yaml))
}

fn node_to_yaml(node: &Node) -> Result<Yaml, TreeError> {
    // Make node buffer
    let mut map = yaml::Hash::new();

    // Set dialogue
    map.insert(
        Yaml::String("dialogue".to_string()),
        Yaml::String(node.dialogue.to_owned()),
    );

    // Set links
    if !node.links.is_empty() {
        let mut links = yaml::Array::new();
        for link in &node.links {
            let yaml_link = link_to_yaml(link)?;
            links.push(yaml_link);
        }
        map.insert(Yaml::String("links".to_string()), Yaml::Array(links));
    }

    let yaml = Yaml::Hash(map);

    Ok(yaml)
}

fn link_to_yaml(link: &Link) -> Result<Yaml, TreeError> {
    let mut map = yaml::Hash::new();
    map.insert(
        Yaml::String(link.to_key.to_owned()),
        Yaml::String(link.dialogue.to_owned()),
    );
    Ok(Yaml::Hash(map))
}

#[cfg(test)]
#[test]
fn test_export() {
    // Test a minimum valid export
    let mut tree = Tree::new();
    let node = Node::new("start", "It's a bad day.");
    tree.nodes.insert("start".to_owned(), node);
    tree.set_root_key("start").unwrap();

    assert!(export(&tree, "examples/dialogue_files/export.convo.yml").is_ok());
}

#[test]
fn test_export_path_exists() {
    use crate::error::ExportError::IO;

    // Make a valid tree
    let mut tree = Tree::new();
    let node = Node::new("start", "It's a bad day.");
    tree.nodes.insert("start".to_owned(), node);
    tree.set_root_key("start").unwrap();

    // Should fail because file path is invalid
    assert!(matches!(export(&tree, "/not/a/path").unwrap_err(), IO(_)));
}

#[test]
fn test_tree_to_source() {
    // Test a minimum valid export
    let mut tree = Tree::new();
    let node = Node::new("start", "It's a bad day.");
    tree.nodes.insert("start".to_owned(), node);
    tree.set_root_key("start").unwrap();

    let source = r#"---
root: start
nodes:
  start:
    dialogue: "It's a bad day.""#;

    // Should be equal
    assert_eq!(source, tree_to_source(&tree).unwrap());
}

#[test]
fn test_tree_to_source_root_exists() {
    // Should fail because root node is never set
    let mut tree = Tree::new();
    let node = Node::new("start", "It's a bad day.");
    tree.nodes.insert("start".to_owned(), node);

    assert!(matches!(
        tree_to_source(&tree).unwrap_err(),
        crate::error::ExportError::Validation(_)
    ));
}

#[test]
fn test_tree_to_source_nodes_exist() {
    // Should fail because nodes do not exist
    let mut tree = Tree::new();
    unsafe { tree.set_root_key_unchecked("start") }

    assert!(matches!(
        tree_to_source(&tree).unwrap_err(),
        crate::error::ExportError::Validation(_)
    ));
}

#[test]
#[ignore = "Waiting on issue #3"]
fn test_tree_to_source_unreachable_nodes() {
    // Should fail because `node2` is an orphan node. It has no parents or links to it.
    let mut tree = Tree::new();
    let node1 = Node::new("1", "It's a bad day.");
    let node2 = Node::new("2", "It's a good day.");
    tree.nodes.insert("1".to_owned(), node1);
    tree.nodes.insert("2".to_owned(), node2);
    tree.set_root_key("1").unwrap();

    assert!(matches!(
        tree_to_source(&tree).unwrap_err(),
        crate::error::ExportError::Validation(_)
    ));

    // This should fail because the root node is a leaf node, e.g. parent becomes unreachable
    let mut tree = Tree::new();
    let mut parent = Node::new("parent", "I am the parent.");
    let child = Node::new("child", "I am the child.");
    Link::link(&mut parent, &child, "I make sure no orphan nodes exist.");
    tree.nodes.insert("parent".to_owned(), parent);
    tree.nodes.insert("child".to_owned(), child);
    tree.set_root_key("child").unwrap();

    assert!(matches!(
        tree_to_source(&tree).unwrap_err(),
        crate::error::ExportError::Validation(_)
    ));
}
#[test]
#[ignore = "Waiting on issue #10"]
fn test_tree_to_source_invalid_links() {
    // Build basic tree
    let mut tree = Tree::new();
    let mut node = Node::new("root", "I am the only node.");
    // Append an invalid link
    let invalid_link = Link::new("invalid", "I am an invalid link");
    node.links.push(invalid_link);
    // Finish tree
    tree.nodes.insert("root".to_owned(), node);
    tree.set_root_key("root").unwrap();

    // Should fail because invalid link exists
    assert!(matches!(
        tree_to_source(&tree).unwrap_err(),
        crate::error::ExportError::Validation(_)
    ));
}
