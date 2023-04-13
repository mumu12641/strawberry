// #[macro_use]
// extern crate lalrpop_util;
// lalrpop_mod!(pub strawberry);

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
    for i in lexer {
        println!("{:?}", i);
    }
}

#[test]
fn test() {
    let a = crate::ast::Feature::Attribute(crate::ast::AttrDecl {
        name: "a".to_string(),
        type_: "b".to_string(),
        init: None,
    });
    assert_eq!(2 + 2, 4);
    println!("{:?}", a);
    let b = crate::token::Token::StringConst("111".to_string());
    println!("{:?}", b);
}
