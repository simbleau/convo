#[macro_use]
extern crate text_io;
use std::{
    io::{self, Write},
    path::Path,
};

fn main() {
    // Select CTree file path
    let path = Path::new("examples/dialogue_files/ex_1.ctree.yml");
    println!("Selected file: {}", path.to_str().unwrap());

    //Parse path to CTree
    print!("Parsing...");
    let ctree = convo::parse(path).unwrap();
    println!("Complete.");
    println!("Starting...\nYou may enter 'Q' to quit anytime.\n");

    // Walk the structure
    let mut current = ctree.root_node().unwrap();
    'walk: loop {
        // Print node dialogue
        println!("{}", current.dialogue);
        if current.links.len() == 0 {
            break 'walk; // Dead end
        }

        // Print node links
        for (id, link) in current.links.iter().enumerate() {
            println!("[{}] {}", id, link.dialogue);
        }
        print!(" > "); // User input prompt
        io::stdout().flush().unwrap(); // Flush before input capture

        // Get user input
        let line: String = read!("{}\n");

        // Handle user input
        if line.trim().eq_ignore_ascii_case("q") {
            break 'walk;
        } else {
            if let Ok(link_id) = line.parse::<usize>() {
                if let Some(link) = current.links.get(link_id) {
                    current = ctree.nodes.get(&link.to).unwrap();
                }
            }
        }
    }

    println!("\nThe conversation has ended.");
}
