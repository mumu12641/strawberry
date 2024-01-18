use std::{cell::RefCell, process::Command};

use owo_colors::OwoColorize;

use crate::{
    cgen::cgen::CodeGenerator, complier::print_err_msg, ctx::CompileContext,
    parser::ast::class::Class,
};

use self::semantic::{SemanticChecker, SemanticError};

pub mod semantic;
pub mod type_checker;

pub fn semantic_check(ctx_ref: RefCell<CompileContext>) -> Result<CompileContext, SemanticError> {
    let mut semantic_checker: SemanticChecker = SemanticChecker::new(ctx_ref.into_inner());
    let result: Result<Vec<Class>, SemanticError> = semantic_checker.check();
    match result {
        Ok(v) => {
            println!(
                "{}",
                "🎺 Congratulations you passped the semantic check!".green()
            );
            return Ok(semantic_checker.ctx);
            // print!("{:?}", v);
            // let ctx = Context::create();
            // let module = ctx.create_module("test");
            // let builder = ctx.create_builder();

            // let mut codegen: IrGenerator<'_> =
            //     IrGenerator::new(v.clone(), &ctx, module, builder, &mut class_table, table);
            // unsafe {
            //     codegen.ir_generate();
            // }
        }
        Err(e) => {
            println!("{}", "❌ Oops, semantic error has occurred!".red());
            if let Some(pos) = e.position {
                print_err_msg(pos, &e.file_name, &e.err_msg);
            } else {
                println!("{}{}", format!("--> ").blue(), e.file_name.blue());
                println!("\t{}", e.err_msg.blue());
            }
            return Err(e);
        }
    }
}
