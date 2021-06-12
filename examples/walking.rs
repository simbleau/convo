#[macro_use]
extern crate text_io;
use std::{env, path::Path};

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = match args.len() {
        2 => Path::new(&args[1]),
        1 => Path::new("examples/dialogue_files/ex1.ctree.yml"), // Default example
        _ => panic!("You must specify only one file."),
    };
    println!("Selected file: {}", path.to_str().unwrap());

    print!("Parsing...");
    let ctree = convo::parse(path).unwrap();
    println!("Complete");

    println!("Starting...\nYou may enter 'Q' to quit anytime.\n");
    let mut current = ctree.root().unwrap();
    loop {
        let node = ctree.nodes.get(current).unwrap();
        println!("{}", node.dialogue);
        if node.links.len() == 0 {
            break;
        }
        for (id, link) in node.links.iter().enumerate() {
            println!("[{}] {}", id, link.dialogue);
        }
        // read until a newline (but not including it)
        let line: String = read!("{}\n");
        if line.eq_ignore_ascii_case("q") {
            break;
        } else {
            let parse = line.parse::<usize>();
            if let Ok(num) = parse {
                if num < node.links.len() {
                    current = &node.links.get(num).unwrap().to;
                }
            }
        }
    }
}
