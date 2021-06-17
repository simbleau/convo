mod building;

pub fn main() {
    // Get a conversation tree to export.
    // (This tree is the tree made in the building example)
    let tree = building::example_tree();

    // Export the tree to a file
    tree.try_export("examples/dialogue_files/ex_export.ctree.yml")
        .unwrap();
}
