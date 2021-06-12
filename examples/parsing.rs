use std::path::Path;

fn main() {
    // Select CTree file path
    let path = Path::new("examples/dialogue_files/ex_min.ctree.yml");
    println!("Selected file: {}", path.to_str().unwrap());

    //Parse path to CTree
    print!("Parsing...");
    let tree = convo::parse(path).unwrap();
    println!("Complete.");

    // Print the data structure
    println!("\n{:#?}", tree);
}
