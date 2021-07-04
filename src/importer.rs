//! A family of functions which parse YAML into [`Tree`]s.

use crate::{
    error::{ImportError, TreeError},
    link::Link,
    node::Node,
    tree::Tree,
};

use std::{fs::File, io::Read, path::Path};
use yaml_rust::{Yaml, YamlLoader};

/// Try to returns a [`Tree`] which is generated from importing a file.
///
/// # Arguments
///
/// * `path` - A path type that references a file to parse from.
/// See also: [example dialogue files](https://github.com/simbleau/convo/tree/main/examples/dialogue_files).
///
/// # Errors
///
/// * An [`ImportError`] will be returned if the source is not valid YAML data or if the tree is not considered legal when parsing.
/// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
///
/// # Examples
///
/// ```
/// use convo::importer;
/// let tree = importer::import("examples/dialogue_files/ex_min.convo.yml").unwrap();
/// ```
pub fn import<P>(path: P) -> Result<Tree, ImportError>
where
    P: AsRef<Path>,
{
    let source = get_file_source(path)?;
    let convo_tree = source_to_tree(&source)?;

    // Return the Tree
    Ok(convo_tree)
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
/// use convo::importer;
/// let source = r#"
/// ---
/// root: start
/// nodes:
///     start:
///         dialogue: I am a recursive node.
///         links:
///             - start: Recurse!
/// "#;
/// let tree = importer::source_to_tree(source).unwrap();
/// ```
pub fn source_to_tree(source: &str) -> Result<Tree, ImportError> {
    // Parse the YAML
    let docs = YamlLoader::load_from_str(source)?;
    if docs.len() != 1 {
        return Err(ImportError::MultipleDocumentsProvided());
    }
    let yaml = &docs[0];

    // Convert YAML to Tree
    let tree = yaml_to_tree(yaml)?;

    Ok(tree)
}

fn get_file_source<P>(path: P) -> Result<String, ImportError>
where
    P: AsRef<Path>,
{
    // Read the file contents
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn yaml_to_tree(yaml: &Yaml) -> Result<Tree, ImportError> {
    // This needs some major cleanup

    let root_key = yaml["root"].as_str().ok_or_else(|| {
        TreeError::Validation("YAML does not contain top-level string key for `root`".into())
    })?;

    let node_map = yaml["nodes"].as_hash().ok_or_else(|| {
        TreeError::Validation("YAML does not contain top-level hash for `nodes`".into())
    })?;

    // Check length of nodes
    if node_map.len() == 0 {
        return Err(TreeError::Validation("Node map has a length of 0".into()).into());
    }

    // Insert nodes
    let mut tree = Tree::new();
    for (key, value) in node_map.iter() {
        let node = yaml_to_node(key, value)?;
        tree.nodes.insert(node.key.clone(), node);
    }

    // Set root and current
    if !tree.nodes.contains_key(root_key) {
        return Err(TreeError::NodeDNE(root_key.into()).into());
    }

    // Safety : Sound code - root node guaranteed to exist, per above
    unsafe {
        tree.set_root_key_unchecked(&root_key);
        tree.set_current_key_unchecked(&root_key);
    }

    Ok(tree)
}

fn yaml_to_node(yaml_key: &Yaml, yaml_data: &Yaml) -> Result<Node, ImportError> {
    // Unwrap name
    let key = yaml_key.as_str().ok_or_else(|| {
        TreeError::Validation(format!("YAML key is not a string: `{:?}`", yaml_key))
    })?;

    // Unwrap data
    let data = yaml_data.as_hash().ok_or_else(|| {
        TreeError::Validation(format!("YAML data is not a hash: '{:?}'", yaml_data))
    })?;

    // Unwrap dialogue
    let dialogue = data
        .get(&Yaml::from_str("dialogue"))
        .ok_or_else(|| {
            TreeError::Validation(format!("YAML does not contain dialogue for `{:?}`", key))
        })?
        .as_str()
        .ok_or_else(|| {
            TreeError::Validation(format!("YAML dialogue is not a string for `{:?}`", key))
        })?;

    let mut node = Node::new(key, dialogue);

    // Check if any links exist
    if let Some(yaml_links) = data.get(&Yaml::from_str("links")) {
        // Unwrap links
        let links = yaml_to_links(yaml_links)?;
        &node.links.extend(links);
    };

    Ok(node)
}

fn yaml_to_links(yaml: &Yaml) -> Result<Vec<Link>, ImportError> {
    // Unwrap link array
    let links = yaml.as_vec().ok_or_else(|| {
        TreeError::Validation(format!("YAML link data is not an array: '{:?}'", yaml))
    })?;

    if links.len() == 0 {
        return Err(TreeError::Validation("Links array has a length of 0".into()).into());
    }

    // Collect links
    let mut link_buf = Vec::<Link>::new();
    for yaml_link in links {
        let yaml_link_hash = yaml_link.as_hash().ok_or_else(|| {
            TreeError::Validation(format!("YAML link is not a hash: '{:?}'", yaml))
        })?;

        for (yaml_to, yaml_dialogue) in yaml_link_hash {
            let to = yaml_to.as_str().ok_or_else(|| {
                TreeError::Validation(format!("YAML link name is not a string:  '{:?}'", yaml))
            })?;
            let dialogue = yaml_dialogue.as_str().ok_or_else(|| {
                TreeError::Validation(format!("YAML link dialogue is not a string for `{:?}`", to))
            })?;
            let link = Link::new(to, dialogue);
            link_buf.push(link);
        }
    }

    Ok(link_buf)
}

#[cfg(test)]
#[test]
fn test_import() {
    let bad_file = "examples/dialogue_files/ex_bad.convo.yml";
    assert!(import(bad_file).is_err());

    let good_file = "examples/dialogue_files/ex_min.convo.yml";
    assert!(import(good_file).is_ok());
}

#[test]
fn test_source_to_tree() {
    // Test a minimal valid source
    let source = r#"---
    root: start
    nodes:
        start:
            dialogue: "It's a bad day."
    "#;
    assert!(source_to_tree(source).is_ok());
}

#[test]
fn test_source_to_tree_root_exists() {
    use crate::error::ImportError::Validation;

    // Invalid: YAML must contain a top-level element called `root`.
    let source = r#"---
    nodes:
        start:
            dialogue: "It's a bad day."
    "#;
    assert!(matches!(source_to_tree(source).unwrap_err(), Validation(_)));

    // Invalid: YAML must contain a top-level element called `root` which points to a real node
    let source = r#"---
    root: abc_123
    nodes:
        start:
            dialogue: "It's a bad day."
    "#;
    assert!(matches!(source_to_tree(source).unwrap_err(), Validation(_)));
}

#[test]
fn test_source_to_tree_nodes_exist() {
    use crate::error::ImportError::Validation;

    // Invalid: `nodes` must contain at least 1 node.
    let source = r#"---
    root: start
    nodes:
    "#;
    assert!(matches!(source_to_tree(source).unwrap_err(), Validation(_)));
}

#[test]
fn test_source_to_tree_attributes() {
    use crate::error::ImportError::Validation;

    // `start` does not contain dialogue.
    let source = r#"---
    root: start
    nodes:
        start:
            links:
            - end: "I'm rudely in a hurry."
        end:
            dialogue: "Ok, let's talk some other time."
    "#;
    assert!(matches!(source_to_tree(source).unwrap_err(), Validation(_)));
}

#[test]
#[ignore = "Waiting on issue #3"]
fn test_source_to_tree_unreachable_nodes() {
    use crate::error::ImportError::Validation;

    // `end` is an orphan node. It is not reachable.
    let source = r#"---
    root: start
    nodes:
        start:
            dialogue: "Hello, how are you?"
        end:
            dialogue: "Ok, let's talk some other time."
    "#;
    assert!(matches!(source_to_tree(source).unwrap_err(), Validation(_)));

    // `end` and `fork` are orphans because the root node (`start`) is a leaf node.
    let source = r#"---
    root: start
    nodes:
        fork:
            dialogue: "I make sure no one is an orphan by being the parent to all."
            links:
                - start: "I link to start"
                - end: "I link to the end"
                - fork: "I even link to myself!"
        start:
            dialogue: "Hello, how are you?"
        end:
            dialogue: "Ok, let's talk some other time."
    "#;
    assert!(matches!(source_to_tree(source).unwrap_err(), Validation(_)));
}

#[test]
#[ignore = "Waiting on issue #10"]
fn test_source_to_tree_invalid_links() {
    use crate::error::ImportError::Validation;

    // `not_a_real_key` is an invalid reference key.
    let source = r#"---
    root: start
    nodes:
        start:
            dialogue: "I am the start node"
            links:
                - start: "I am valid and link to myself"
                - not_a_real_key: "I do not link to a valid key"
    "#;

    assert!(matches!(source_to_tree(source).unwrap_err(), Validation(_)));
}
