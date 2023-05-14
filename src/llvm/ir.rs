extern crate llvm_sys as llvm;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

use crate::{grammar::ast::class::Class, utils::table::Tables};
pub struct IrGenerator {
    pub context: *mut llvm::LLVMContext,
    pub module: *mut llvm::LLVMModule,
    pub builder: *mut llvm::LLVMBuilder,
    pub classes: Vec<Class>,
}

impl IrGenerator {
    pub unsafe fn new(file_name: String, v: Vec<Class>) -> IrGenerator {
        let context_ = llvm::core::LLVMContextCreate();
        IrGenerator {
            context: context_,
            module: llvm::core::LLVMModuleCreateWithName(file_name.as_ptr() as *const _),
            builder: llvm::core::LLVMCreateBuilderInContext(context_),
            classes: v,
        }
    }

    /// generate constant
    /// generate class template
    /// generate class method dispatch table
    pub unsafe fn ir_generate(&self, tables: &Tables) {
        // define type(e.g int64)
        // define function type (return type, param type and so on)
        // define entry and block
        // emit ir code
        self.const_generate(tables);
        llvm::core::LLVMDumpModule(self.module);
        //     // Clean up. Values created in the context mostly get cleaned up there.
        llvm::core::LLVMDisposeBuilder(self.builder);
        llvm::core::LLVMDisposeModule(self.module);
        llvm::core::LLVMContextDispose(self.context);
    }

    unsafe fn const_generate(&self, tables: &Tables) {
    }
}

// unsafe {
//     // Set up a context, module and builder in that context.
//     let context: *mut llvm::LLVMContext = llvm::core::LLVMContextCreate();
//     let module: *mut llvm::LLVMModule =
//         llvm::core::LLVMModuleCreateWithName(b"test\0".as_ptr() as *const _);
//     let builder: *mut llvm::LLVMBuilder = llvm::core::LLVMCreateBuilderInContext(context);

//     // Get the type signature for void nop(void);
//     // Then create it in our module.
//     let int_type = llvm::core::LLVMInt64TypeInContext(context);
//     let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
//     let function =
//         llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);

//     let entry_name = CString::new("entry").unwrap();
//     let bb =
//         llvm::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
//     llvm::core::LLVMPositionBuilderAtEnd(builder, bb);

//     // The juicy part: construct a `LLVMValue` from a Rust value:
//     let int_value: u64 = 42;
//     let int_value = llvm::core::LLVMConstInt(int_type, int_value, 0);

//     llvm::core::LLVMBuildRet(builder, int_value);

//     // Instead of dumping to stdout, let's write out the IR to `out.ll`
//     let out_file = CString::new("out.ll").unwrap();
//     llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

//     // Clean up. Values created in the context mostly get cleaned up there.
//     llvm::core::LLVMDisposeBuilder(builder);
//     llvm::core::LLVMDisposeModule(module);
//     llvm::core::LLVMContextDispose(context);
// }
