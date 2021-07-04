use std::path::Path;

pub fn main() {
    // Importing a tree file
    let path_in = Path::new("examples/dialogue_files/ex_min.convo.yml");
    println!("Selected input path: {}", path_in.to_str().unwrap());
    print!("Importing...");
    let tree = convo::importer::import(path_in).unwrap();
    println!("Complete.");

    // Print the data structure
    println!("\n{:#?}\n", tree);

    // Export the tree to a new file
    let path_out = Path::new("examples/dialogue_files/export.convo.yml");
    println!("Selected output path: {}", path_in.to_str().unwrap());
    print!("Exporting...");
    tree.try_export(path_out).unwrap();
    println!("Complete.");
}
