use std::{cell::RefCell, fs::File, io::Read};

use owo_colors::OwoColorize;

use crate::{
    complier::print_err_msg,
    ctx::CompileContext,
    lexer::{self, lexer::Lexer, Position},
    strawberry,
};

use self::ast::class::Class;

pub mod ast;

pub fn parse_file(files: Vec<String>, ctx_ref: &RefCell<CompileContext>) {
    let main_file = "./src/main.st".to_string();
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
        ctx_ref.borrow_mut().preprocess(content);
        ctx_ref.borrow_mut().file_name = file_name;
        let lexer: Lexer = lexer::lexer_parse(&ctx_ref);
        parse(lexer, &ctx_ref);
    }
    println!(
        "{}",
        "🎉 Congratulations you passped the syntax analysis!".green()
    );
}

fn parse(lexer: Lexer, ctx: &RefCell<CompileContext>) {
    let program = strawberry::ProgramParser::new().parse(lexer);

    match program {
        Ok(mut v) => {
            ctx.borrow_mut().classes.append(&mut v.1);
        }
        Err(e) => {
            let err = format!(
                "❌ Oops, syntax error has occurred in {}!",
                &ctx.borrow().file_name
            );
            println!("{}", err.red());
            print!("{}", "Err: ".red());
            match e {
                lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                    let err = format!("There is an unrecognized token <{:?}> !", token.1,);
                    println!("{}", err.red());
                    print_err_msg(
                        Position::new(token.0, token.2),
                        &ctx.borrow().file_name,
                        &format!("Maybe you can try {} here!", expected.join(" or ")),
                    );
                }
                lalrpop_util::ParseError::ExtraToken { token } => {
                    let err = format!(
                        "There is an extra token <{:?}> at {}:{}:{}",
                        token.1,
                        &ctx.borrow().file_name,
                        token.0,
                        token.2,
                    );
                    println!("{}", err.red());
                }
                _ => {}
            }
            return;
        }
    }
}
