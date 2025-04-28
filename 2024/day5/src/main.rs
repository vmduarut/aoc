use std::fs;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");
    //println!("{:?}", file_content);
    for i in file_content.lines() {
        println!("{}", i);
    }
}
