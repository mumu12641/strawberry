use std::fs::File;
use std::io::prelude::*;
mod lexer;
mod token;

fn main() {
    let mut file = File::open("src/test.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");

    // // let contents = fs::read_to_string().expect("Should have been able to read the file");
    println!("With text:\n{content}");

    let lexer = lexer::Lexer::new(&content);
    for i in lexer {
        println!("in main {:?}", i);
    }
}
