#![feature(const_trait_impl)]
#[macro_use]
extern crate lalrpop_util;
extern crate clap;

lalrpop_mod!(pub strawberry);
use clap::{Arg, ColorChoice};
use grammar::lexer::Lexer;
use owo_colors::OwoColorize;
use semantic::semantic::{SemanticChecker, SemanticError};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use utils::table::{self, ClassTable, Tables};

use crate::cgen::cgen::CodeGenerator;
use crate::grammar::ast::class::Class;

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

const LOGO: &str = r#"
______     __                                       __                                               
/      \   |  \                                     |  \                                              
|  $$$$$$\ _| $$_     ______   ______   __   __   __ | $$____    ______    ______    ______   __    __ 
| $$___\$$|   $$ \   /      \ |      \ |  \ |  \ |  \| $$    \  /      \  /      \  /      \ |  \  |  \
\$$    \  \$$$$$$  |  $$$$$$\ \$$$$$$\| $$ | $$ | $$| $$$$$$$\|  $$$$$$\|  $$$$$$\|  $$$$$$\| $$  | $$
_\$$$$$$\  | $$ __ | $$   \$$/      $$| $$ | $$ | $$| $$  | $$| $$    $$| $$   \$$| $$   \$$| $$  | $$
|  \__| $$  | $$|  \| $$     |  $$$$$$$| $$_/ $$_/ $$| $$__/ $$| $$$$$$$$| $$      | $$      | $$__/ $$
\$$    $$   \$$  $$| $$      \$$    $$ \$$   $$   $$| $$    $$ \$$     \| $$      | $$       \$$    $$
 \$$$$$$     \$$$$  \$$       \$$$$$$$  \$$$$$\$$$$  \$$$$$$$   \$$$$$$$ \$$       \$$       _\$$$$$$$
                                                                                            |  \__| $$
                                                                                             \$$    $$
                                                                                              \$$$$$$ 
"#;

fn main() {
    handle_args();
}

fn handle_args() {
    println!("\n{}", LOGO.green());
    let mut cmd = clap::Command::new("Strawberry")
        .color(ColorChoice::Auto)
        .version("0.1-beta")
        .about("A toy object-oriented programming language")
        .subcommand(clap::Command::new("build").about("Build the current project directory"))
        .subcommand(
            clap::Command::new("new")
                .about("Create a new empty project folder")
                .arg(
                    Arg::new("name")
                        .required(true)
                        .help("The name of the new project"),
                ),
        );
    let matches = cmd.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        println!("{}", matches.get_one::<String>("name").unwrap());
        create_project_folder(matches.get_one::<String>("name").unwrap());
    } else if let Some(_) = matches.subcommand_matches("build") {
        if let Ok(paths) = fs::read_dir("./src") {
            let mut files: Vec<_> = vec![];

            for path in paths {
                files.insert(0, path.unwrap().path().to_str().unwrap().to_string());
            }
            compile(files);
        } else {
            let err = format!("❌ Failed to build because the current directory is not a strawberry project, try strawberry new example");
            println!("{}", err.red());
        }
    } else {
        let _ = cmd.print_long_help();
    }
}

fn compile(files: Vec<String>) {
    let mut all_classes: Vec<Class> = vec![];

    // init
    let mut table = table::Tables::new();
    table.string_table.insert("".to_string());
    table.string_table.insert("Object".to_string());
    table.string_table.insert("%s".to_string());
    table.int_table.insert("0".to_string());
    let mut class_table = ClassTable::new();

    // install constants
    class_table.install_basic_class();

    for file_name in files {
        let mut file = File::open(&file_name).unwrap();
        let mut content = String::new();
        // file.read_to_string(&mut content).expect("error");
        if let Ok(_) = file.read_to_string(&mut content) {
        } else {
            println!("{}", "❌ Some unexpected errors occurred, maybe you can solve it by recreating the project".red());
            return;
        }
        let lexer: Lexer = Lexer::new(&content, &mut table, &file_name);
        let program = strawberry::ProgramParser::new().parse(lexer);
        match program {
            Ok(mut v) => {
                all_classes.append(&mut v);
            }
            Err(e) => {
                let err = format!("❌ Oops, syntax error has occurred in {}!", &file_name);
                println!("{}", err.red());
                println!("{}", "Err: ".red());
                println!("{:?}", e.red());
                return;
            }
        }
    }

    println!(
        "{}",
        "🎉 Congratulations you passped the syntax analysis!".green()
    );

    let mut semantic_checker: SemanticChecker = SemanticChecker::new(all_classes.clone());
    if DEBUG {
        println!("Res: {:?}", &all_classes);
    }
    let result: Result<Vec<Class>, SemanticError> = semantic_checker.check(&mut class_table);
    match result {
        Ok(v) => {
            println!(
                "{}",
                "🎺 Congratulations you passped the semantic check!".green()
            );
            let mut asm_file = std::fs::File::create("./build/a.s").expect("create failed");
            let mut cgen = CodeGenerator::new(v, &mut class_table, table, &mut asm_file);
            cgen.code_generate();
            Command::new("gcc")
                .arg("-no-pie")
                .arg("-static")
                .arg("./build/a.s")
                .arg("-o")
                .arg("./build/a.out")
                .spawn()
                .expect("gcc command failed to start");
            println!("{}", "🔑 Congratulations you successfully generated assembly code, please execute ./build/a.out in your shell!".green());
        }
        Err(e) => {
            println!("{}", "❌ Oops, semantic error has occurred!".red());
            println!("{}", e.err_msg.red());
        }
    }
}

fn create_project_folder(name: &str) {
    let path = Path::new(name);

    if path.exists() {
        println!("Error: {} already exists", name);
        return;
    }

    fs::create_dir(path).expect("Failed to create project folder");
    fs::create_dir(path.join("src")).expect("Failed to create project src folder");
    fs::create_dir(path.join("build")).expect("Failed to create project build folder");

    let mut file = File::create(path.join("src/main.st")).expect("Failed to create main.st");

    file.write(
        b"class Main { \n\tfun main() -> Int { \n\t\tprint(\"Hello world!\"); \n\t\treturn 0; \n\t};\n};",
    
    ).unwrap();
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
    table.string_table.insert("".to_string());
    table.string_table.insert("Object".to_string());
    table.string_table.insert("%s".to_string());
    table.int_table.insert("0".to_string());
    let mut class_table = ClassTable::new();

    // install constants
    class_table.install_basic_class();
    let lexer: Lexer = Lexer::new(&content, &mut table, "test.st");
    // for i in lexer {
    //     println!("{:?}", i);
    // }
    let program = strawberry::ProgramParser::new().parse(lexer);
    match program {
        Ok(v) => {
            let mut semantic_checker: SemanticChecker = SemanticChecker::new(v.clone());
            // if DEBUG {let result: Result<Vec<Class>, SemanticError> =
            let result = semantic_checker.check(&mut class_table);
            match result {
                Ok(v) => {
                    println!("{:?}", v);
                }
                _ => {}
            }
        }
        Err(_) => todo!(),
    };
}
