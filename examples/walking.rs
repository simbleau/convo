#[macro_use]
extern crate text_io;
use std::{
    io::{self, Write},
    path::Path,
};

use convo::CTree;

fn main() {
    // Select CTree file path
    let path = Path::new("examples/dialogue_files/ex_1.ctree.yml");
    println!("Selected file: {}", path.to_str().unwrap());

    //Parse path to CTree
    print!("Parsing...");
    let ctree = convo::parser::parse(path).unwrap();
    println!("Complete.");
    println!("Starting...\nYou may enter 'Q' to quit anytime.\n");

    // Walk the CTree
    walk(ctree);
}

fn walk(mut ctree: CTree) {
    // Walk the structure
    'walk: while let Some(current) = ctree.current_node() {
        // Print node dialogue
        println!("{}", current.dialogue);

        // End if there's no links to choose
        if current.links.is_empty() {
            break 'walk; // Dead end
        }

        // Print node links
        for (id, link) in current.links.iter().enumerate() {
            println!("[{}] {}", id, link.dialogue);
        }

        // Get user input
        print!(" > "); // User input prompt
        io::stdout().flush().unwrap(); // Flush before input capture
        let line: String = read!("{}\n"); // Capture

        // Handle user input
        if line.trim().eq_ignore_ascii_case("q") {
            break 'walk; // User quit
        } else {
            if let Ok(link_id) = line.parse::<usize>() {
                if let Some(link) = current.links.get(link_id) {
                    let link_key = link.to.clone();
                    ctree.set_current(&link_key).unwrap();
                }
            }
        }
    }
}
