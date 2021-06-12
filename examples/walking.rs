#[macro_use]
extern crate text_io;
use std::path::Path;

fn main() {
    // Select CTree file path
    let path = Path::new("examples/dialogue_files/ex_min.ctree.yml");
    println!("Selected file: {}", path.to_str().unwrap());

    //Parse path to CTree
    print!("Parsing...");
    let ctree = convo::parse(path).unwrap();
    println!("Complete.");
    println!("Starting...\nYou may enter 'Q' to quit anytime.\n");

    // Walk the structure
    let mut current = ctree.root().unwrap();
    'walk: loop {
        let node = ctree.nodes.get(current).unwrap();

        // Print node dialogue
        println!("{}", node.dialogue);
        if node.links.len() == 0 {
            break 'walk; // Dead end
        }

        // Print node links
        for (id, link) in node.links.iter().enumerate() {
            println!("[{}] {}", id, link.dialogue);
        }

        // Handle user input
        let line: String = read!("{}\n");
        if line.eq_ignore_ascii_case("q") {
            break 'walk;
        } else {
            if let Ok(num) = line.parse::<usize>() {
                if num < node.links.len() {
                    current = &node.links.get(num).unwrap().to;
                }
            }
        }
    }
}
