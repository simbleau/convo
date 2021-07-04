#[macro_use]
extern crate text_io;
use std::{
    io::{self, Write},
    path::Path,
};

use convo::Tree;

fn main() {
    // Select convo file to walk
    let path_in = Path::new("examples/dialogue_files/ex_1.convo.yml");
    println!("Selected input path: {}", path_in.to_str().unwrap());

    // Import convo file
    print!("Importing...");
    let tree = convo::importer::import(path_in).unwrap();
    println!("Complete.");

    // Walk the Tree
    println!("Starting...\nYou may enter 'Q' to quit anytime.\n");
    walk(tree);
}

fn walk(mut tree: Tree) {
    // Walk the structure
    'walk: while let Some(current) = tree.current_node() {
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
                    let link_key = link.to_key.clone();
                    tree.set_current_key(&link_key).unwrap();
                }
            }
        }
    }
}
