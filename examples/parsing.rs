use std::{env, path::Path};

fn main() {
    let args: Vec<_> = env::args().collect();

    let path = match args.len() {
        2 => Path::new(&args[1]),
        1 => Path::new("examples/dialogue_files/example.ctree.yml"), // Default example
        _ => panic!("You must specify only one file."),
    };
    println!("Selected file: {}", path.to_str().unwrap());

    println!("Parsing...");
    let ctree = convo::parse(path).unwrap();
    println!("Completed.\n\n{:#?}", ctree);
}
