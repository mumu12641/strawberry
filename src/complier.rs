use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fs::{self, metadata, File};
use std::io::Read;
use std::process::Command;

use clap::parser;
use owo_colors::OwoColorize;
use simple_home_dir::home_dir;

use crate::cgen::cgen;
use crate::ctx::CompileContext;
use crate::lexer::lexer::Lexer;
use crate::lexer::{self, Position};
use crate::parser::ast::class::Class;
use crate::semantic::semantic::{SemanticChecker, SemanticError};
use crate::utils::table::ClassTable;
use crate::{semantic, strawberry};

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
    let mut ctx = CompileContext::new();
    let ctx_ref = RefCell::new(ctx);
    crate::parser::parse_file(files, &ctx_ref);
    let result = semantic::semantic_check(ctx_ref);
    if let Ok(ctx) = result {
        crate::llvm::llvm_ir(ctx);
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
