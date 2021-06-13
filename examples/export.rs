use convo::CTree;
use convo_lib::{link::Link, node::Node};

pub fn main() {
    // Build a basic convo tree.
    let mut tree = CTree::new();
    let mut node1 = Node::new("start", "I am the root node!");
    let node2 = Node::new("end", "I am the last node!");
    Link::link(&mut node1, &node2, "I link start to end!");
    tree.nodes.insert(node1.key.clone(), node1);
    tree.nodes.insert(node2.key.clone(), node2);
    tree.set_root("start").unwrap();

    // Export the tree to a file
    CTree::try_export(&tree, "examples/dialogue_files/ex_export.ctree.yml").unwrap();
}
