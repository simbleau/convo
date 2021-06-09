use std::{env, path::Path};

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() == 2, "You must specify a file.");

    let path = Path::new(&args[1]);
    println!("Selected file: {}", path.to_str().unwrap());

    println!("Parsing...");
    let ctree = convo::parser::parse(path).unwrap();
    println!("Completed.\n\n{:?}", ctree);
}
