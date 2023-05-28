#![feature(const_trait_impl)]
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub strawberry);
use grammar::lexer::Lexer;

use semantic::semantic::{SemanticChecker, SemanticError};
use std::{fs::File, process::Command};
use std::io::prelude::*;
use utils::table::{self, ClassTable, Tables};

use crate::cgen::cgen::CodeGenerator;

mod cgen;
mod grammar;
mod llvm;
mod semantic;
mod utils;
const STRING: &str = "String";
const OBJECT: &str = "Object";
const INT: &str = "Int";
const BOOL: &str = "Bool";
const SELF: &str = "self";
const EMPTY: (usize, usize) = (0, 0);

const DEBUG: bool = false;

fn main() {
    // get input file
    let mut file = File::open("src/helloworld.st").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");
    // println!("{content}");

    // init
    let mut table = table::Tables::new();
    table.string_table.insert("".to_string());
    table.string_table.insert("Object".to_string());
    table.int_table.insert("0".to_string());
    let mut class_table = ClassTable::new();

    // install constants
    class_table.install_basic_class();

    // start compiler
    let lexer: Lexer = Lexer::new(&content, &mut table, "helloworld.st");
    println!("Congratulations you passped the lexical analysis!");
    let program = strawberry::ProgramParser::new().parse(lexer);
    // if DEBUG {
    // print_table(&table);
    // }
    match program {
        Ok(v) => {
            println!("Congratulations you passped the syntax analysis!");
            let mut semantic_checker: SemanticChecker = SemanticChecker::new(v.clone());
            if DEBUG {
                println!("Res: {:?}", &v);
            }
            let result: Result<bool, SemanticError> = semantic_checker.check(&mut class_table);
            match result {
                Ok(_) => {
                    println!("Congratulations you passped the semantic check!");
                    let mut asm_file = std::fs::File::create("test.s").expect("create failed");
                    let mut cgen =
                        CodeGenerator::new(v.clone(), &mut class_table, table, &mut asm_file);
                    cgen.code_generate();
                    Command::new("gcc")
                        .arg("-no-pie")
                        .arg("-static")
                        .arg("./test.s")
                        .spawn()
                        .expect("ls command failed to start");
                }
                Err(e) => {
                    println!();
                    println!("Oops, semantic error has occurred!");
                    println!("{}", e.err_msg);
                }
            }
        }
        Err(e) => {
            println!("Oops, syntax error has occurred!");
            println!("Err: {:?}", e);
        }
    }
}

// for debug
fn print_table(table: &Tables) {
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
}

#[test]
fn test() {
    let mut file = File::open("src/helloworld.st").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");
    // println!("{content}");

    // init
    let mut table = table::Tables::new();
    let lexer: Lexer = Lexer::new(&content, &mut table, "test.st");
    for i in lexer {
        println!("{:?}", i);
    }
}

// Ok((5, Return((5, 14)), 14))
// Ok((5, Identifier("test", (5, 19)), 19))
// Ok((5, Lparen, 20))
// Ok((5, Identifier("b", (5, 21)), 21))
// Ok((5, Rparen, 22))
// Ok((5, Semicolon, 23))
