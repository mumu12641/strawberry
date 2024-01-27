use std::cell::RefCell;

use inkwell::context::Context;

use crate::ctx::CompileContext;

use self::{env::Env, ir::IrGenerator};

pub mod ast_ir;
pub mod env;
pub mod ir;
pub mod types;
pub mod utils;

pub fn llvm_ir(ctx: CompileContext) {
    let llvm_ctx = Context::create();
    let module = llvm_ctx.create_module("test");
    let builder = llvm_ctx.create_builder();
    let env = Env::new();
    let env = RefCell::new(env);
    let mut codegen: IrGenerator<'_> = IrGenerator::new(ctx, &llvm_ctx, module, builder);
    unsafe {
        codegen.ir_generate(env);
    }
}
