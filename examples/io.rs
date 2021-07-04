use std::path::Path;

pub fn main() {
    // Importing a tree file
    let path = Path::new("examples/dialogue_files/ex_min.convo.yml");
    println!("Selected file: {}", path.to_str().unwrap());

    //Parse path to Tree
    print!("Parsing...");
    let tree = convo::parser::parse(path).unwrap();
    println!("Complete.");

    // Print the data structure
    println!("\n{:#?}\n", tree);

    // Export the tree to a new file
    print!("Exporting...");
    tree.try_export("examples/dialogue_files/export.convo.yml")
        .unwrap();
    println!("Complete.");
}
