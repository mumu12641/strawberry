#![feature(const_trait_impl)]
#![allow(warnings)]
#[macro_use]
extern crate lalrpop_util;
extern crate clap;

lalrpop_mod!(pub strawberry);

use clap::{Arg, ColorChoice};
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::path::Path;
use utils::table::{self};
use lazy_static::lazy_static;

mod cgen;
mod complier;
mod ctx;
// mod grammar;
mod lexer;
mod parser;
// mod llvm;
mod semantic;
// mod ty;
mod utils;

const STRING: &str = "String";
const OBJECT: &str = "Object";
const INT: &str = "int";
const INTEGER: &str = "Integer";
const BOOL: &str = "Bool";
const SELF: &str = "self";
const VOID: &str = "Void";
const PRIMSLOT: &str = "PrimSlot";
const RUNTIME_ERR: &str = "Some runtime errors occurred and the program has crashed! \\n";
// const EMPTY_POSITION: (usize, usize) = (0, 0);

const INT_CONST_VAL_OFFSET: usize = 24;
const BOOL_CONST_VAL_OFFSET: usize = 24;
const STRING_CONST_VAL_OFFSET: usize = 24;
const DISPATCH_TABLE_OFFSET: usize = 16;
const NULL_TAG_OFFSET: usize = 8;
const FIELD_BASIC_OFFSET: usize = 24;

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
        let msg = format!("🎉 Congratulations, you successfully created the project, please use cd ./{}, and then use strawberry build to build the project!", matches.get_one::<String>("name").unwrap());
        println!("{}", msg.green());
        create_project_folder(matches.get_one::<String>("name").unwrap());
    } else if let Some(_) = matches.subcommand_matches("build") {
        complier::build();
    } else {
        let _ = cmd.print_long_help();
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
