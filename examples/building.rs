use convo::CTree;
use convo_lib::{link::Link, node::Node};

fn main() {
    let mut tree = CTree::new();

    // Declare the root key
    let root_key = "start";

    // Create some nodes
    let mut node1 = Node::new(root_key, "I am the root node!");
    let node2 = Node::new("end", "I am the last node!");

    // Link node1 -> node 2
    Link::link(&mut node1, &node2, "I link start to end!");

    // Populate the tree
    tree.nodes.insert(node1.key.clone(), node1);
    tree.nodes.insert(node2.key.clone(), node2);

    // Set the root node (which also sets current to root if current is None)
    tree.set_root(root_key).unwrap();

    // Print the data structure
    println!("\n{:#?}", tree);
}
