#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub strawberry);

use std::fs::File;
use std::io::prelude::*;
mod ast;
mod lexer;
mod token;

fn main() {
    let mut file = File::open("src/test.st").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");

    println!("With text:\n{content}");

    let lexer = lexer::Lexer::new(&content);
    let program = strawberry::ProgramParser::new().parse(lexer);
        match program {
            Ok(v) => println!("Res: {:?}", v),
            Err(e) => {
                println!("Err: {:?}", e);
                for token_tup in lexer::Lexer::new(&content) {
                    println!("{:?}", token_tup);
                }
            },
        }
}

#[test]
fn test() {}
