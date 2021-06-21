//! A family of functions which parse YAML into [`CTree`]s.

use crate::{
    error::{ParseError, TreeError},
    link::Link,
    node::Node,
    tree::CTree,
};

use std::{fs::File, io::Read, path::Path};
use yaml_rust::{Yaml, YamlLoader};

/// Try to returns a [`CTree`] which is generated from parsing a file.
///
/// # Arguments
///
/// * `path` - A path type that references a file to parse from.
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
/// use convo::parser;
/// let tree = parser::parse("examples/dialogue_files/ex_min.ctree.yml").unwrap();
/// ```
pub fn parse<P>(path: P) -> Result<CTree, ParseError>
where
    P: AsRef<Path>,
{
    let source = get_file_source(path)?;
    let convo_tree = source_to_ctree(&source)?;

    // Return the CTree
    Ok(convo_tree)
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
/// use convo::parser;
/// let source = r#"
/// ---
/// root: start
/// nodes:
///     start:
///         dialogue: I am a recursive node.
///         links:
///             - start: Recurse!
/// "#;
/// let tree = parser::source_to_ctree(source).unwrap();
/// ```
pub fn source_to_ctree(source: &str) -> Result<CTree, ParseError> {
    // Parse the YAML
    let docs = YamlLoader::load_from_str(source)?;
    if docs.len() != 1 {
        return Err(ParseError::MultipleDocumentsProvided());
    }
    let yaml = &docs[0];

    // Convert YAML to CTree
    let ctree = yaml_to_ctree(yaml)?;

    Ok(ctree)
}

fn get_file_source<P>(path: P) -> Result<String, ParseError>
where
    P: AsRef<Path>,
{
    // Read the file contents
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn yaml_to_ctree(yaml: &Yaml) -> Result<CTree, ParseError> {
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

    let mut tree = CTree::new();
    node_map
        .iter()
        .flat_map(|(key, value)| yaml_to_node(key, value))
        .for_each(|node| {
            tree.nodes.insert(node.key.clone(), node);
        });

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

fn yaml_to_node(yaml_key: &Yaml, yaml_data: &Yaml) -> Result<Node, ParseError> {
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

fn yaml_to_links(yaml: &Yaml) -> Result<Vec<Link>, ParseError> {
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
