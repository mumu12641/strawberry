use std::{cell::RefCell, process::Command};

use owo_colors::OwoColorize;

use crate::{cgen::cgen::CodeGenerator, ctx::CompileContext};

mod ast;
pub mod cgen;

pub fn code_gen(ctx: CompileContext) {
    let mut asm_file = std::fs::File::create("./build/a.s").expect("create failed");
    let mut cgen = CodeGenerator::new(ctx, &mut asm_file);
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
