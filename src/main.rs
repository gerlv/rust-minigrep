use std::env;
use std::fs;


fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Query: {query}");
    println!("File path: {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Poem to exist");
    
    println!("File contents:\n{contents}");
}
