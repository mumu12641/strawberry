use owo_colors::OwoColorize;

use crate::{
    complier::print_err_msg,
    ctx::CompileContext,
    lexer::{lexer::Lexer, Position},
    strawberry,
};

use self::ast::class::Class;

pub mod ast;

pub fn parse(lexer: Lexer, ctx: &mut CompileContext) {
    // let mut all_classes: Vec<Class> = vec![];
    let program = strawberry::ProgramParser::new().parse(lexer);

    match program {
        Ok(mut v) => {
            ctx.classes.append(&mut v.1);
        }
        Err(e) => {
            let err = format!("❌ Oops, syntax error has occurred in {}!", &ctx.file_name);
            println!("{}", err.red());
            print!("{}", "Err: ".red());
            match e {
                lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                    let err = format!("There is an unrecognized token <{:?}> !", token.1,);
                    println!("{}", err.red());
                    print_err_msg(
                        Position::new(token.0, token.2),
                        &ctx.file_name,
                        &format!("Maybe you can try {} here!", expected.join(" or ")),
                    );
                }
                lalrpop_util::ParseError::ExtraToken { token } => {
                    let err = format!(
                        "There is an extra token <{:?}> at {}:{}:{}",
                        token.1, &ctx.file_name, token.0, token.2,
                    );
                    println!("{}", err.red());
                }
                _ => {}
            }
            return;
        }
    }
    println!(
        "{}",
        "🎉 Congratulations you passped the syntax analysis!".green()
    );
}
