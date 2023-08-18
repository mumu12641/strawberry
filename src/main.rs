#![feature(const_trait_impl)]
#[macro_use]
extern crate lalrpop_util;
extern crate clap;

lalrpop_mod!(pub strawberry);
use clap::{Arg, ColorChoice};
use grammar::lexer::Lexer;
use grammar::lexer::Position;
use owo_colors::OwoColorize;
use semantic::semantic::{SemanticChecker, SemanticError};

use simple_home_dir::*;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use utils::table::{self, ClassTable};

use crate::cgen::cgen::CodeGenerator;
use crate::grammar::ast::class::Class;
use crate::utils::util::fix_offset;

mod cgen;
mod grammar;
mod llvm;
mod semantic;
mod utils;
const STRING: &str = "String";
const OBJECT: &str = "Object";
const INT: &str = "Int";
const RAW_INT: &str = "int";
const BOOL: &str = "Bool";
const SELF: &str = "self";
const VOID: &str = "Void";
const PRIMSLOT: &str = "PrimSlot";
const RUNTIME_ERR: &str = "Some runtime errors occurred and the program has crashed! \\n";
const EMPTY: (usize, usize) = (0, 0);

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
const logo_img:&str =r#"
                                                            
                                                            
                                                            
            ##                                              
             %###   ((((                                    
            //( ###((//(((((((//////                        
            //////##//////////////                          
       &//(((///(((////((*/(((((((((                        
      //((////#(((((((((******/((((((                       
  //////////(((((((((((((((((((((((((## /    #%             
     #####(((((((((((((((##((((((((((((((*** ##             
    #####((((((((##(((((((((((((((((((((((((*## ****        
    ######((((((((((((((((((((((((((((((((*/(#((((((&       
     ###(((((((((((((((##(((((((((((###/***//**(((((((((    
     ####(((((##((((((((((((((((((((((****//***#(((((((     
      ####((((((((((((((((((##(((####**********(##((((((    
        ###(#(((((((##(((((((((((((((********#(((((((((     
         ####(((((((((((((##((((((###/*****((##(((((((      
            ####((##(((((((((((((((((***/##(((((((((        
               ####(((((((((((((((((***(((##(((((           
                    &#####(((((((((***((((((((              
                              ((   *((((                    
                                                            
                                                            
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
        let msg = format!   ("🎉 Congratulations, you successfully created the project, please use cd ./{}, and then use strawberry build to build the project!", matches.get_one::<String>("name").unwrap());
        println!("{}", msg.green());
        create_project_folder(matches.get_one::<String>("name").unwrap());
    } else if let Some(_) = matches.subcommand_matches("build") {
        let mut curr_path = "./src".to_string();
        let mut path_flag = true;
        let home_dir = home_dir().unwrap().into_os_string().into_string().unwrap();
        let mut files: Vec<String> = vec![
            format!("{}/.strawberry/std/Object.st", home_dir),
            format!("{}/.strawberry/std/Int.st", home_dir),
            format!("{}/.strawberry/std/String.st", home_dir),
            format!("{}/.strawberry/std/Bool.st", home_dir),
            format!("{}/.strawberry/std/Void.st", home_dir),
        ];

        while path_flag {
            path_flag = false;
            if let Ok(paths) = fs::read_dir(&curr_path) {
                // paths.

                for path in paths {
                    if let Ok(d) = &path {
                        let path_name = d.path().to_str().unwrap().to_string();
                        if metadata(&path_name).unwrap().is_dir() {
                            path_flag = true;
                            curr_path = path_name.to_string();
                        } else {
                            files.insert(0, path_name);
                        }
                    }
                }
            } else {
                let err = format!("❌ Failed to build because the current directory is not a strawberry project, try strawberry new example");
                println!("{}", err.red());
            }
        }
        compile(files);
    } else {
        let _ = cmd.print_long_help();
    }
}

fn compile(files: Vec<String>) {
    let mut all_classes: Vec<Class> = vec![];
    let main_file = "./src/main.st".to_string();

    // init
    let mut table = table::Tables::new();
    table.string_table.insert("".to_string());
    table.string_table.insert("Object".to_string());
    table.string_table.insert("%s".to_string());
    table.string_table.insert("%d".to_string());
    table.string_table.insert(RUNTIME_ERR.to_string());
    table.int_table.insert("0".to_string());
    let mut class_table = ClassTable::new();

    if !files.contains(&main_file) {
        let err = format!("❌ There is no main.st in your src directory!");
        println!("{}", err.red());
    }

    for file_name in files {
        let mut file = File::open(&file_name).unwrap();
        let mut content = String::new();
        if let Ok(_) = file.read_to_string(&mut content) {
        } else {
            println!("{}", "❌ Some unexpected errors occurred, maybe you can solve it by recreating the project".red());
            return;
        }
        content = fix_offset(content);
        let lexer: Lexer = Lexer::new(&content, &mut table, &file_name);
        let program = strawberry::ProgramParser::new().parse(lexer);
        match program {
            Ok(mut v) => {
                all_classes.append(&mut v.1);
            }
            Err(e) => {
                let err = format!("❌ Oops, syntax error has occurred in {}!", &file_name);
                println!("{}", err.red());
                print!("{}", "Err: ".red());
                match e {
                    lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                        let err = format!("There is an unrecognized token <{:?}> !", token.1,);
                        println!("{}", err.red());
                        print_err_msg(
                            (token.0, token.2),
                            &file_name,
                            &format!("Maybe you can try {} here!", expected.join(" or ")),
                        );
                    }
                    lalrpop_util::ParseError::ExtraToken { token } => {
                        let err = format!(
                            "There is an extra token <{:?}> at {}:{}:{}",
                            token.1, &file_name, token.0, token.2,
                        );
                        println!("{}", err.red());
                    }
                    _ => {}
                }
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
                .arg("-m64")
                .arg("./build/a.s")
                .arg("-o")
                .arg("./build/a.out")
                .spawn()
                .expect("gcc command failed to start");
            println!("{}", "🔑 Congratulations you successfully generated assembly code, please execute ./build/a.out in your shell!".green());
        }
        Err(e) => {
            println!("{}", "❌ Oops, semantic error has occurred!".red());
            if let Some(pos) = e.position {
                print_err_msg(pos, &e.file_name, &e.err_msg);
            } else {
                println!("{}{}", format!("--> ").blue(), e.file_name.blue());
                println!("\t{}", e.err_msg.blue());
            }
        }
    }
}

fn print_err_msg(pos: Position, file_name: &String, err_msg_: &String) {
    let (line, off) = pos;
    let mut err_file = File::open(file_name).unwrap();
    let mut err_content = String::new();
    let mut lines;
    let err_msg = format!("--> {}:{}:{}", file_name, line, off);
    err_file.read_to_string(&mut err_content).expect("error");
    lines = err_content.lines();
    println!("{}", err_msg.blue());
    println!("{0:<4}{1:<4}", "".to_string(), format!("|").blue());
    print!("{0:<4}{1:<4}", line.blue(), format!("|").blue());
    println!("{}", lines.nth(line - 1).unwrap().blue());
    print!(
        "{0:<4}{1:<4}{2:<off$}",
        "".to_string(),
        format!("|").blue(),
        "".to_string()
    );
    println!("{}{}", format!("^ ").red(), err_msg_.red());
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

#[test]
fn test() {
    let mut file = File::open("src/helloworld.st").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");
    // println!("{:?}", content.as_bytes());

    // init
    let mut table = table::Tables::new();
    table.string_table.insert("".to_string());
    table.string_table.insert("Object".to_string());
    table.string_table.insert("%s".to_string());
    table.int_table.insert("0".to_string());
    let mut class_table = ClassTable::new();

    // install constants

    //*   class_table.install_basic_class();
    let lexer: Lexer = Lexer::new(&content, &mut table, "test.st");
    // for i in lexer {
    //     println!("{:?}", i);
    // }
    let program = strawberry::ProgramParser::new().parse(lexer);
    match program {
        Ok(v) => {
            let mut semantic_checker: SemanticChecker = SemanticChecker::new(v.1.clone());
            // if DEBUG {let result: Result<Vec<Class>, SemanticError> =
            let result = semantic_checker.check(&mut class_table);
            match result {
                Ok(v) => {
                    println!("{:?}", v);
                }
                Err(e) => {
                    println!("{}", e.err_msg);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
}

#[test]
fn some_test() {
    let a = "%d as";
    let s = a.replace("%d", "1");
    println!("{}", s);

    let binding = "Foo".to_string();
    let binding12 = "Bar".to_string();
    let string_list: Vec<&String> = vec![&binding, &binding12];
    let v: Vec<String> = string_list.iter().map(|x| x.to_string()).collect();
    let joined = v.join("-");
    assert_eq!("Foo-Bar", joined);
    println!("{}", joined)
}
