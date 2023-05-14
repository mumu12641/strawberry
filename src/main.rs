#![feature(const_trait_impl)]
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub strawberry);
use grammar::lexer::Lexer;
use semantic::semantic::{SemanticChecker, SemanticError};
use std::fs::File;
use std::io::prelude::*;
use utils::table::{self, ClassTable, Tables};

use crate::llvm::ir::IrGenerator;
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

fn main() {
    // get input file
    let mut file = File::open("src/test.st").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");
    // println!("{content}");

    // init
    let mut table = table::Tables::new();
    let mut class_table = ClassTable::new();

    // install constants
    class_table.install_basic_class();

    // start compiler
    let lexer: Lexer = Lexer::new(&content, &mut table, "test.st");
    let program = strawberry::ProgramParser::new().parse(lexer);

    print_table(&table);
    match program {
        Ok(v) => {
            let mut semantic_checker: SemanticChecker = SemanticChecker::new(v.clone());
            println!("Res: {:?}", &v);
            let result: Result<bool, SemanticError> = semantic_checker.check(&mut class_table);
            match result {
                Ok(_) => {
                    println!("Congratulations you passed the semantic check!");
                    unsafe {
                        let ir = IrGenerator::new("test.st".to_string(), v.clone());
                        ir.ir_generate(&table);
                    }
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
