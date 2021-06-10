use convo_lib::{link::Link, node::Node, tree::CTree};

extern crate convo_lib;

#[test]
// Basic print out of all nodes and links - Not a great test
fn test_print() {
    let mut tree = CTree::default();

    // Build some nodes
    let root_key = "root";
    let mut node1 = Node::new(root_key, "How are you?");
    let node2 = Node::new("end", "Don't be late!");

    // Link node1 -> node 2
    Link::link(&mut node1, &node2, "Sorry, I'm in a hurry!");

    // Populate the tree
    tree.nodes.insert(node1.key.clone(), node1);
    tree.nodes.insert(node2.key.clone(), node2);

    // Set node1 as root
    tree.set_root(root_key).unwrap();

    // Print root node:
    println!("Node root: [{}]", tree.root().unwrap());

    // Print them all out
    for (_, node) in &tree.nodes {
        println!("Node [{}]: '{}'", node.key, node.dialogue);
        for link in &node.links {
            let linked_search = tree.nodes.get(link.to());
            if let Some(link_node) = linked_search {
                println!(
                    "Link [{}]->[{}]: '{}'",
                    node.key,
                    link_node.key,
                    link.dialogue()
                );
            }
        }
    }
}
