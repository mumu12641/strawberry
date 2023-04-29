#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub strawberry);

use std::fs::File;
use std::io::prelude::*;
mod ast;
mod lexer;
mod table;
mod token;

fn main() {
    let mut file = File::open("src/test.st").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");

    println!("{content}");

    let mut table = table::Tables::new();
    let lexer: lexer::Lexer = lexer::Lexer::new(&content, &mut table);

    let program = strawberry::ProgramParser::new().parse(lexer);
    println!("***String Table***");
    for i in &table.string_table {
        println!("{i}");
    }
    println!();
    println!("***Int Table***");
    for i in &table.int_table {
        println!("{i}");
    }
    println!();
    println!("***Id Table***");
    for i in &table.id_table {
        println!("{i}");
    }
    println!();
    match program {
        Ok(v) => println!("Res: {:?}", v),
        Err(e) => {
            println!("Err: {:?}", e);
        }
    }
}

#[test]
fn test() {}
