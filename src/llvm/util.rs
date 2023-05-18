use llvm_sys::core::*;
use llvm_sys::{core::LLVMConstInt, prelude::LLVMValueRef};
pub unsafe fn get_llvm_int_const_u32(val: u32) -> LLVMValueRef {
    LLVMConstInt(LLVMInt32Type(), val.into(), 1)
}
pub unsafe fn get_llvm_int_const_u64(val: u64) -> LLVMValueRef {
    LLVMConstInt(LLVMInt64Type(), val.into(), 1)
}
