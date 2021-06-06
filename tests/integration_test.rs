use convo::dialogue_tree::DialogueTree;

extern crate convo;

#[test]
// Basic print out of all nodes and links - Not a great test
fn test_print() {
    let tree = DialogueTree::example_tree();
    for (_, node) in tree.nodes() {
        println!("Node [{}]: '{}'", node.key(), node.description());
        for link in node.links() {
            let linked_search = tree.nodes().get(link.to());
            if let Some(link_node) = linked_search {
                println!(
                    "Link [{}]->[{}]: '{}'",
                    node.key(),
                    link_node.key(),
                    link.description()
                );
            }
        }
    }
}
