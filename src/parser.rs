extern crate yaml_rust;

use crate::tree::CTree;
use convo_lib::{link::Link, node::Node};
use std::{fs::File, io::Read, path::Path};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub enum ParseError {
    IO(std::io::Error),
    Scan(yaml_rust::ScanError),
    Validation(String),
}
impl From<std::io::Error> for ParseError {
    fn from(item: std::io::Error) -> Self {
        ParseError::IO(item)
    }
}
impl From<yaml_rust::ScanError> for ParseError {
    fn from(item: yaml_rust::ScanError) -> Self {
        ParseError::Scan(item)
    }
}
impl From<String> for ParseError {
    fn from(item: String) -> Self {
        ParseError::Validation(item)
    }
}
impl From<&str> for ParseError {
    fn from(item: &str) -> Self {
        ParseError::Validation(item.to_owned())
    }
}

fn get_file_contents<P>(path: P) -> Result<String, ParseError>
where
    P: AsRef<Path>,
{
    // Read the file contents
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn get_ctree(source: String) -> Result<CTree, ParseError> {
    // Parse the YAML
    let docs = YamlLoader::load_from_str(&source)?;
    if docs.len() != 1 {
        return Err("Only one YAML document must be provided".into());
    }
    let yaml = &docs[0];

    // Convert YAML to CTree
    let ctree = yaml_to_ctree(yaml)?;

    Ok(ctree)
}

fn yaml_to_ctree(yaml: &Yaml) -> Result<CTree, ParseError> {
    // This needs some major cleanup

    let root = yaml["root"].as_str().ok_or_else(|| "The root is missing")?;
    println!("Root: {:?}", root);

    let node_map = yaml["nodes"]
        .as_hash()
        .ok_or_else(|| "The nodes are missing")?;

    // Check length of nodes
    if node_map.len() == 0 {
        return Err(ParseError::Validation(
            "At least one node must be given".into(),
        ));
    }

    let mut tree = CTree::default();
    node_map
        .iter()
        .flat_map(|(key, value)| yaml_to_node(key, value))
        .for_each(|node| {
            tree.nodes_mut().insert(node.key().clone(), node);
        });

    // Set root and current
    let root_node = tree
        .nodes()
        .get(root)
        .ok_or_else(|| format!("Root node DNE for {:?}", root))?
        .key()
        .clone();
    tree.set_root(root_node)?;
    tree.reset()?;

    Ok(tree)
}

fn yaml_to_node(yaml_key: &Yaml, yaml_data: &Yaml) -> Result<Node, ParseError> {
    // Unwrap name
    let key = yaml_key
        .as_str()
        .ok_or_else(|| format!("Missing node name for '{:?}'", yaml_key))?;

    // Unwrap data
    let data = yaml_data
        .as_hash()
        .ok_or_else(|| format!("Missing node data for '{:?}'", yaml_data))?;

    // Unwrap dialogue
    let dialogue = data
        .get(&Yaml::from_str("dialogue"))
        .ok_or_else(|| format!("Dialogue missing for '{:?}'", key))?
        .as_str()
        .ok_or_else(|| format!("Dialogue not a string for '{:?}'", key))?;

    let mut node = Node::new(key, dialogue);

    // Check if any links exist
    if let Some(yaml_links) = data.get(&Yaml::from_str("links")) {
        // Unwrap links
        let links = yaml_to_links(yaml_links)?;
        node.links_mut().extend(links);
    };

    Ok(node)
}

fn yaml_to_links(yaml_links: &Yaml) -> Result<Vec<Link>, ParseError> {
    // Unwrap link hashmap
    let links = yaml_links
        .as_vec()
        .ok_or_else(|| format!("Links not an array for '{:?}'", yaml_links))?
        .first()
        .ok_or_else(|| format!("Links empty for '{:?}'", yaml_links))?
        .as_hash()
        .ok_or_else(|| format!("Links not a hash for '{:?}'", yaml_links))?
        .iter();

    // Collect links
    let mut link_buf = Vec::<Link>::new();
    for (yaml_to, yaml_dialogue) in links {
        let to = yaml_to
            .as_str()
            .ok_or_else(|| format!("Link name missing for '{:?}'", yaml_links))?;
        let dialogue = yaml_dialogue
            .as_str()
            .ok_or_else(|| format!("Links dialogue missing for '{:?}'", to))?;
        let link = Link::new(to, dialogue);
        link_buf.push(link);
    }

    Ok(link_buf)
}

pub fn parse<P>(path: P) -> Result<CTree, ParseError>
where
    P: AsRef<Path>,
{
    let source = get_file_contents(path)?;
    let convo_tree = get_ctree(source)?;

    // Return the CTree
    Ok(convo_tree)
}
