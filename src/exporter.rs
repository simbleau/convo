//! A family of functions which export [`CTree`]s into YAML data.

use crate::{
    error::{ExportError, TreeError},
    link::Link,
    node::Node,
    tree::CTree,
};

use std::{fs::File, io::Write, path::Path};
use yaml_rust::{yaml, Yaml, YamlEmitter};

/// Try to save a [`CTree`] as a file.
///
/// # Arguments
///
/// * `tree` - A [`CTree`] that will be saved in a file.
///
/// # Errors
///
/// * An [`ExportError`] will be returned if the tree breaks validation rules or incurs issues saving.
/// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
///
/// # Examples
///
/// ```
/// use convo::{parser, exporter};
/// let tree = parser::parse("examples/dialogue_files/ex_min.ctree.yml").unwrap();
/// // Make a copy of the file
/// exporter::export(&tree, "examples/dialogue_files/export.ctree.yml").unwrap();
/// ```
pub fn export<P>(tree: &CTree, path: P) -> Result<(), ExportError>
where
    P: AsRef<Path>,
{
    let source = ctree_to_source(tree).map_err(|err| ExportError::Tree(err))?;

    // Write file
    let mut file = File::create(path)?;
    file.write_all(source.as_bytes())?;

    Ok(())
}

/// Try to returns a [`String`] which is generated as YAML from a [`CTree`].
///
/// # Arguments
///
/// * `tree` - A [`CTree`] that will be returned as YAML data.
///
/// # Errors
///
/// * An [`ExportError`] will be returned if the tree breaks validation rules.
/// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
///
/// # Examples
///
/// ```
/// use convo::{parser, exporter};
/// let source = r#"---
/// root: start
/// nodes:
///   start:
///     dialogue: I am a recursive node.
///     links:
///       - start: Recurse!"#;
/// let tree = parser::source_to_ctree(source).unwrap();
/// let source2 = exporter::ctree_to_source(&tree).unwrap();
/// assert_eq!(source, source2);
/// ```
pub fn ctree_to_source(tree: &CTree) -> Result<String, TreeError> {
    let yaml = ctree_to_yaml(&tree)?;
    // Convert to source text
    let mut writer = String::new();
    let mut emitter = YamlEmitter::new(&mut writer);
    emitter.compact(true);
    emitter
        .dump(&yaml)
        .map_err(|_err| TreeError::Validation("YAML Dump error".to_string()))?;

    Ok(writer)
}

fn ctree_to_yaml(tree: &CTree) -> Result<Yaml, TreeError> {
    let root_key = tree.root_key().ok_or_else(|| TreeError::RootNotSet())?;

    // Check length of nodes
    if tree.nodes.len() == 0 {
        return Err(TreeError::Validation(
            "At least one node must be given".into(),
        ));
    }

    let mut node_map = yaml::Hash::new();
    for (key, node) in &tree.nodes {
        let yaml_key = Yaml::String(key.to_owned());
        let yaml_node = node_to_yaml(&node)?;
        node_map.insert(yaml_key, yaml_node);
    }

    let mut yaml = yaml::Hash::new();
    yaml.insert(
        Yaml::String("root".to_string()),
        Yaml::String(root_key.to_owned()),
    );
    yaml.insert(Yaml::String("nodes".to_string()), Yaml::Hash(node_map));

    Ok(Yaml::Hash(yaml))
}

fn node_to_yaml(node: &Node) -> Result<Yaml, TreeError> {
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
