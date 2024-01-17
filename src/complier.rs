use std::fs::{self, metadata, File};
use std::io::Read;
use std::process::Command;

use clap::parser;
use owo_colors::OwoColorize;
use simple_home_dir::home_dir;

use crate::cgen::cgen::CodeGenerator;
use crate::ctx::CompileContext;
use crate::lexer::lexer::Lexer;
use crate::lexer::{self, Position};
use crate::parser::ast::class::Class;
use crate::semantic::semantic::{SemanticChecker, SemanticError};
use crate::strawberry;
use crate::utils::table::ClassTable;

pub fn build() {
    let mut curr_path = "./src".to_string();
    let mut path_flag = true;
    let home_dir = home_dir().unwrap().into_os_string().into_string().unwrap();
    let mut files: Vec<String> = vec![
        format!("{}/.strawberry/std/Object.st", home_dir),
        format!("{}/.strawberry/std/Integer.st", home_dir),
        format!("{}/.strawberry/std/String.st", home_dir),
        format!("{}/.strawberry/std/Bool.st", home_dir),
        format!("{}/.strawberry/std/Void.st", home_dir),
    ];

    while path_flag {
        path_flag = false;
        if let Ok(paths) = fs::read_dir(&curr_path) {
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
            let err = format!("❌ Failed to build because the current directory is not a strawberry project, try \"strawberry new example\"");
            println!("{}", err.red());
        }
    }
    compile(files);
}

fn compile<'a>(files: Vec<String>) {
    let mut all_classes: Vec<Class> = vec![];
    let main_file = "./src/main.st".to_string();
    let mut ctx = CompileContext::new();

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
        ctx.preprocess(content);
        ctx.file_name = file_name;

        // let lexer: Lexer = Lexer::new(&mut ctx);
        let lexer: Lexer = lexer::lexer_parse(&mut ctx);
        crate::parser::parse(lexer, &mut ctx);
        // let program = strawberry::ProgramParser::new().parse(lexer);

        // match program {
        //     Ok(mut v) => {
        //         all_classes.append(&mut v.1);
        //     }
        //     Err(e) => {
        //         let err = format!("❌ Oops, syntax error has occurred in {}!", &ctx.file_name);
        //         println!("{}", err.red());
        //         print!("{}", "Err: ".red());
        //         match e {
        //             lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
        //                 let err = format!("There is an unrecognized token <{:?}> !", token.1,);
        //                 println!("{}", err.red());
        //                 print_err_msg(
        //                     Position::new(token.0, token.2),
        //                     &ctx.file_name,
        //                     &format!("Maybe you can try {} here!", expected.join(" or ")),
        //                 );
        //             }
        //             lalrpop_util::ParseError::ExtraToken { token } => {
        //                 let err = format!(
        //                     "There is an extra token <{:?}> at {}:{}:{}",
        //                     token.1, &ctx.file_name, token.0, token.2,
        //                 );
        //                 println!("{}", err.red());
        //             }
        //             _ => {}
        //         }
        //         return;
        //     }
        // }
    }

    // println!(
    //     "{}",
    //     "🎉 Congratulations you passped the syntax analysis!".green()
    // );
    let mut semantic_checker: SemanticChecker = SemanticChecker::new(ctx.classes.clone());
    // if DEBUG {
    //     println!("Res: {:?}", &all_classes);
    // }
    let result: Result<Vec<Class>, SemanticError> = semantic_checker.check(&mut ctx.class_table);
    match result {
        Ok(v) => {
            println!(
                "{}",
                "🎺 Congratulations you passped the semantic check!".green()
            );
            // let ctx = Context::create();
            // let module = ctx.create_module("test");
            // let builder = ctx.create_builder();

            // let mut codegen: IrGenerator<'_> =
            //     IrGenerator::new(v.clone(), &ctx, module, builder, &mut class_table, table);
            // unsafe {
            //     codegen.ir_generate();
            // }

            let mut asm_file = std::fs::File::create("./build/a.s").expect("create failed");
            let mut cgen = CodeGenerator::new(v, &mut ctx.class_table, ctx.tables, &mut asm_file);
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

pub fn print_err_msg(pos: Position, file_name: &String, err_msg_: &String) {
    let line = pos.row;
    let off = pos.column;
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
