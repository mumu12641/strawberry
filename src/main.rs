#![feature(const_trait_impl)]
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub strawberry);
use grammar::lexer::Lexer;
use inkwell::context::Context;
use semantic::semantic::{SemanticChecker, SemanticError};
use std::fs::File;
use std::io::prelude::*;
use utils::table::{self, ClassTable, Tables};

use crate::{cgen::cgen::CodeGenerator, llvm::ir::IrGenerator};

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
                    println!("Congratulations you passped the semantic check!");
                    // unsafe {
                    // let context = Context::create();
                    // let module = context.create_module("test.st");
                    // let ir = IrGenerator {
                    //     classes: v.clone(),
                    //     context: &context,
                    //     module,
                    //     builder: context.create_builder(),
                    // };
                    // ir.ir_generate(&table);
                    // }
                    let mut asm_file = std::fs::File::create("test.s").expect("create failed");
                    // file.write_all("简单教程".as_bytes()).expect("write failed");
                    let mut cgen = CodeGenerator {
                        classes: v.clone(),
                        tables: table,
                        asm_file: &mut asm_file,
                    };
                    cgen.code_generate();

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
