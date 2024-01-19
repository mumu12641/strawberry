use inkwell::context::Context;

use crate::ctx::CompileContext;

use self::ir::IrGenerator;

pub mod ast_ir;
pub mod env;
pub mod ir;
pub mod types;
pub mod utils;

pub fn llvm_ir(ctx: CompileContext) {
    let llvm_ctx = Context::create();
    let module = llvm_ctx.create_module("test");
    let builder = llvm_ctx.create_builder();

    let mut codegen: IrGenerator<'_> = IrGenerator::new(ctx, &llvm_ctx, module, builder);
    unsafe {
        codegen.ir_generate();
    }
}
