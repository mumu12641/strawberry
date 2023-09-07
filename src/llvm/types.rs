use inkwell::types::BasicTypeEnum;

use crate::grammar::ast::class::MethodDecl;

use super::ir::IrGenerator;

pub trait EmitLLVMType {
    fn emit_llvm_type(&self, ir_genrator: &IrGenerator) -> BasicTypeEnum {
        unreachable!()
    }
}

impl EmitLLVMType for MethodDecl {
    fn emit_llvm_type(&self, ir_genrator: &IrGenerator) -> BasicTypeEnum {
        // self.param
        // BasicTypeEnum::PointerType(
        //     ir_genrator
        //         .ctx
        //         .void_type()
        //         .fn_type(params_type, false)
        //         .ptr_type(AddressSpace::default()),
        // )
        unreachable!()
    }
}
