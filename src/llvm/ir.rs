extern crate llvm_sys as llvm;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

use llvm::LLVMVisibility;
use llvm_sys::core::*;
use strawberry::{array_size, c_str};

use crate::{
    grammar::ast::class::Class,
    llvm::util::{get_llvm_int_const_u32, get_llvm_int_const_u64},
    utils::table::{self, Tables},
};
pub struct IrGenerator {
    pub context: *mut llvm::LLVMContext,
    pub module: *mut llvm::LLVMModule,
    pub builder: *mut llvm::LLVMBuilder,
    pub classes: Vec<Class>,
}

impl IrGenerator {
    pub unsafe fn new(file_name: String, v: Vec<Class>) -> IrGenerator {
        let context_ = LLVMContextCreate();
        IrGenerator {
            context: context_,
            module: LLVMModuleCreateWithName(file_name.as_ptr() as *const _),
            builder: LLVMCreateBuilderInContext(context_),
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

        // build main function
        let main_func_type = LLVMFunctionType(LLVMInt16Type(), ptr::null_mut(), 0, 0);
        let main_func = LLVMAddFunction(self.module, c_str!("main"), main_func_type);
        let main_block = LLVMAppendBasicBlockInContext(self.context, main_func, c_str!("main"));
        LLVMPositionBuilderAtEnd(self.builder, main_block);
        LLVMBuildRetVoid(self.builder);

        self.const_generate(tables);

        LLVMDumpModule(self.module);
        let out_file = CString::new("out.ll").unwrap();
        LLVMPrintModuleToFile(self.module, out_file.as_ptr(), ptr::null_mut());
        // Clean up. Values created in the context mostly get cleaned up there.
        LLVMDisposeBuilder(self.builder);
        LLVMDisposeModule(self.module);
        LLVMContextDispose(self.context);
    }

    unsafe fn const_generate(&self, tables: &Tables) {
        println!("*****start build constant*****");
        let mut int_const_index = 0;
        let mut string_const_index = 0;
        let int_const_struct_type = LLVMStructTypeInContext(
            self.context,
            [
                LLVMInt32Type(), // value
                LLVMInt64Type(), // disp table
                LLVMInt32Type(), // class tag
                LLVMInt32Type(), // object size
            ]
            .as_ptr() as *mut _,
            4,
            0,
        );

        for i in &tables.int_table {
            println!("debug");
            // let int_const = LLVMAddGlobal(
            //     self.module,
            //     LLVMInt32Type(),
            //     c_str!(int_const_index.to_string() + "_int_const"),
            // );
            let chars: Vec<char> = i.chars().collect();
            let int_const_struct = LLVMAddGlobal(
                self.module,
                int_const_struct_type,
                c_str!(int_const_index.to_string() + "_int_const"),
            );
            let int_const_struct_val = LLVMConstStruct(
                [
                    get_llvm_int_const_u32(i.parse::<u32>().unwrap()),
                    get_llvm_int_const_u64(114514),
                    get_llvm_int_const_u32(1),
                    get_llvm_int_const_u32(20),
                ]
                .as_ptr() as *mut _,
                4,
                1,
            );

            LLVMSetVisibility(int_const_struct, LLVMVisibility::LLVMProtectedVisibility);
            LLVMSetInitializer(int_const_struct, int_const_struct_val);
            int_const_index += 1;
        }

        for i in &tables.string_table {
            let str_const_struct_type = LLVMStructTypeInContext(
                self.context,
                [
                    LLVMArrayType([LLVMInt8Type(); 6].as_ptr() as *mut _, 6), // value
                    // LLVMInt64Type(), //value
                    LLVMInt32Type(), // length
                    LLVMInt64Type(), // disp table
                    LLVMInt32Type(), // class tag
                    LLVMInt32Type(), // object size
                ]
                .as_ptr() as *mut _,
                5,
                0,
            );
            let str_const_struct = LLVMAddGlobal(
                self.module,
                str_const_struct_type,
                c_str!(int_const_index.to_string() + "_str_const_struct"),
            );
            let str_const_struct_val = LLVMConstStruct(
                [
                    // get_llvm_int_const_u64(114514),
                    LLVMConstArray(LLVMInt8Type(), [], 6)
                    get_llvm_int_const_u32(i.len() as u32),
                    get_llvm_int_const_u64(114514),
                    get_llvm_int_const_u32(2),
                    get_llvm_int_const_u32(114514),
                ]
                .as_ptr() as *mut _,
                4,
                1,
            );
            LLVMSetVisibility(str_const_struct, LLVMVisibility::LLVMProtectedVisibility);
            LLVMSetInitializer(str_const_struct, str_const_struct_val);
            string_const_index += 1;
            // LLVMBuildStructGEP2(self.builder, LLVMInt64Type(), Pointer, Idx, Name)
        }
        println!("*****end build constant*****");
    }

    // let s = LLVMAddGlobal(self.module, strcut_type, c_str!("test_struct"));
    // LLVMSetVisibility(s, LLVMVisibility::LLVMProtectedVisibility);
    // let int_const_val: *mut llvm::LLVMValue = LLVMConstInt(LLVMInt64Type(), 64, 1);

    // let s_val = LLVMConstStruct([int_const_val, int_const_val].as_ptr() as *mut _, 2, 1);
    // // let s_val = LLVMConstInt(LLVMInt64Type(), i.parse::<u64>().unwrap(), 1);
    // LLVMSetInitializer(s, s_val);

    // let global = LLVMAddGlobal(self.module, LLVMInt16Type(), c_str!("global"));
    // LLVMSetVisibility(global,LLVMVisibility::LLVMProtectedVisibility);
    // LLVMConstNull(LLVMInt16Type());
    // let const_int = LLVMConstInt(LLVMInt16Type(), 145, 1);
    // LLVMSetInitializer(global,const_int);

    // let main_func_type = LLVMFunctionType(LLVMInt16Type(), ptr::null_mut(), 0, 0);
    // let main_func = LLVMAddFunction(self.module, c_str!("main"), main_func_type);
    // let main_block = LLVMAppendBasicBlockInContext(self.context, main_func, c_str!("main"));
    // LLVMPositionBuilderAtEnd(self.builder, main_block);

    // // main's function body
    // // let hello_world_str =
    // //     LLVMBuildGlobalStringPtr(self.builder, c_str!("hello,world."), c_str!(""));

    // for str_ in &tables.string_table {
    //     LLVMBuildGlobalStringPtr(self.builder, c_str!(str_.clone()), c_str!("test_str"));
    // }
    // let malloc_ptr = LLVMBuildMalloc(self.builder, LLVMInt16Type(), c_str!("malloc_test"));
    // let const_int = LLVMConstInt(LLVMInt16Type(), 12, 1);
    // LLVMBuildStore(self.builder, const_int, malloc_ptr);
    // let ret_val = LLVMBuildLoad2(self.builder, LLVMInt16Type(), global, c_str!("ret_val"));
    // // LLVMBuildRetVoid(self.builder);
    // LLVMBuildRet(self.builder, ret_val);
    fn test() {
        unsafe {
            // Set up a context, module and builder in that context.
            let context: *mut llvm::LLVMContext = llvm::core::LLVMContextCreate();
            let module: *mut llvm::LLVMModule =
                llvm::core::LLVMModuleCreateWithName(b"test\0".as_ptr() as *const _);
            let builder: *mut llvm::LLVMBuilder = llvm::core::LLVMCreateBuilderInContext(context);

            // Get the type signature for void nop(void);
            // Then create it in our module.
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
            let function =
                llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);

            let entry_name = CString::new("entry").unwrap();
            let bb =
                llvm::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
            llvm::core::LLVMPositionBuilderAtEnd(builder, bb);

            // The juicy part: construct a `LLVMValue` from a Rust value:
            let int_value: u64 = 42;
            let int_value = llvm::core::LLVMConstInt(int_type, int_value, 0);
            // common types
            let void_type = LLVMVoidTypeInContext(context);
            let i8_type = LLVMIntTypeInContext(context, 8);
            let i8_pointer_type = LLVMPointerType(i8_type, 0);

            // declare that there's a `void log(i8*)` function in the environment
            // but don't provide a block (aka body) so that it in the wasm module
            // it'll be imported
            let log_func_type =
                LLVMFunctionType(void_type, [i8_pointer_type].as_ptr() as *mut _, 1, 0);
            let log_func = LLVMAddFunction(module, c_str!("log"), log_func_type);
            let hello_world_str =
                LLVMBuildGlobalStringPtr(builder, c_str!("hello,world."), c_str!(""));
            let log_args = [hello_world_str].as_ptr() as *mut _;
            // calling `log("hello, world.")`
            // LLVMBuildCall(builder, log_func, log_args, 1, c_str!(""));
            // LLVMBuildCall2(self.builder, log_func_type, log_func, c_str!(""), 1, c_str!(""));
            LLVMBuildCall2(
                builder,
                log_func_type,
                log_func,
                log_args,
                1,
                c_str!("result"),
            );

            llvm::core::LLVMBuildRet(builder, int_value);

            // Instead of dumping to stdout, let's write out the IR to `out.ll`
            let out_file = CString::new("out.ll").unwrap();

            // LLVMSetTarget
            // LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

            llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

            // Clean up. Values created in the context mostly get cleaned up there.
            llvm::core::LLVMDisposeBuilder(builder);
            llvm::core::LLVMDisposeModule(module);
            llvm::core::LLVMContextDispose(context);
        }
    }
}
