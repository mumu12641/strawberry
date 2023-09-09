use std::{borrow::BorrowMut, collections::HashMap, fmt::format, hash::Hash, ops::Deref};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, StructType},
    values::{BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace,
};

use crate::{
    grammar::ast::{
        class::{self, Class, Feature, ParamDecl},
        Type,
    },
    utils::table::{ClassTable, Tables},
    DISPATCH_TABLE_OFFSET, OBJECT,
};

use super::types::LLVMType;
#[derive(Debug)]
/// class prototype
/// class method table prototype
/// class init method
/// class constructor method
/// expressions
pub struct IrGenerator<'ctx> {
    pub classes: Vec<Class>,
    pub ctx: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub class_table: &'ctx mut ClassTable,
    pub tables: Tables,
    // pub env: Env<'ctx>,
}
#[derive(Debug)]
pub struct Env<'a> {
    pub function_table: HashMap<Type, &'a FunctionType<'a>>,
}

impl Env<'_> {
    pub fn new() -> Self {
        Env {
            function_table: HashMap::new(),
        }
    }
}

impl<'ctx> IrGenerator<'ctx> {
    pub fn new(
        classes: Vec<Class>,
        ctx: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        class_table: &'ctx mut ClassTable,
        tables: Tables,
    ) -> Self {
        IrGenerator {
            classes,
            ctx,
            module,
            builder,
            class_table,
            tables, // env: Env::new(),
        }
    }

    pub unsafe fn ir_generate(&self) {
        //* for placeholder */
        let mut placeholder: HashMap<&Type, StructType> = HashMap::new();
        for class in &self.classes {
            placeholder.insert(&class.name, self.ctx.opaque_struct_type(&class.name));
        }

        //* generate method ir */
        //* first is init_method*/
        self.gen_init_method();

        //* generate class prototypes */
        for class in &self.classes {
            class.emit_llvm_type(self, *placeholder.get(&class.name).unwrap());
        }

        self.gen_methods();

        //* generate main function */
        self.gen_main();
    }

    fn gen_main(&self) {
        let main_function =
            self.module
                .add_function("main", self.ctx.i32_type().fn_type(&[], false), None);
        let main_entry_block = self.ctx.append_basic_block(main_function, "entry");
        let zero = self.ctx.i32_type().const_int(0, false);
        self.builder.position_at_end(main_entry_block);
        let _ = self
            .builder
            .build_malloc(self.module.get_struct_type("Main").unwrap(), "m");
        self.builder.build_return(Some(&zero));
        let _ = self.module.print_to_file("./test.ll");
    }

    fn gen_init_method(&self) {
        //* for place holder */
        for class in &self.classes {
            self.module.add_function(
                &format!("{}.init", &class.name),
                self.get_funtion_type(
                    &[self
                        .get_llvm_type(LLVMType::from_string_to_llvm_type(&class.name))
                        .into()],
                    None,
                ),
                None,
            );
        }
    }

    fn gen_methods(&self) {
        //* emit init method */
        for class in &self.classes {
            let init_method = self.get_function(format!("{}.init", &class.name));
            let entry_block = self.ctx.append_basic_block(init_method, "entry");
            self.builder.position_at_end(entry_block);

            //* call parents' init method */
            if &class.name != OBJECT {
                let parent = self.class_table.get_parent(&class.name);
                self.builder.build_call(
                    self.get_function(format!("{}.init", parent)),
                    &[init_method.get_first_param().unwrap().into()],
                    "a",
                );
            }
            //* store method table to class */
            let m = self.builder.build_struct_gep(
                init_method.get_first_param().unwrap().into_pointer_value(),
                0,
                "A",
            );
            self.builder.build_store(
                m.unwrap(),
                self.module
                    .get_global(&format!("{}_dispatch_table", &class.name))
                    .unwrap(),
            );

            self.builder.build_return(None);
        }
        for class in &self.classes {
            for f in &class.features {
                if let Feature::Method(method) = f {
                    let mut params: Vec<BasicMetadataTypeEnum> = method
                        .param
                        .deref()
                        .iter()
                        .map(|param| {
                            self.get_llvm_type(LLVMType::from_string_to_llvm_type(&param.1))
                                .into()
                        })
                        .collect();
                    params.insert(
                        0,
                        self.get_llvm_type(LLVMType::from_string_to_llvm_type(&class.name))
                            .into(),
                    );
                    self.module.add_function(
                        &format!("{}.{}", &class.name, method.name),
                        self.get_funtion_type(params.as_slice(), None),
                        None,
                    );
                }
            }
        }
    }

    pub fn get_llvm_type(&self, llvm_type: LLVMType) -> BasicTypeEnum<'ctx> {
        match llvm_type {
            LLVMType::I32 => BasicTypeEnum::IntType(self.ctx.i32_type()),
            LLVMType::StructType { type_ } => {
                if let Some(t) = self.module.get_struct_type(&type_) {
                    return BasicTypeEnum::PointerType(
                        BasicTypeEnum::StructType(t).ptr_type(AddressSpace::default()),
                    );
                }
                unreachable!()
            }
        }
    }

    fn get_funtion_type(
        &self,
        params: &[BasicMetadataTypeEnum<'ctx>],
        return_type: Option<BasicMetadataTypeEnum<'ctx>>,
    ) -> FunctionType<'ctx> {
        match return_type {
            Some(return_type_) => match return_type_ {
                BasicMetadataTypeEnum::IntType(type_) => type_.fn_type(params, false),
                BasicMetadataTypeEnum::StructType(type_) => type_.fn_type(params, false),
                BasicMetadataTypeEnum::PointerType(type_) => type_.fn_type(params, false),
                _ => unreachable!(),
            },
            None => self.ctx.void_type().fn_type(params, false),
        }
    }
    pub fn get_function(&self, function_name: String) -> FunctionValue<'ctx> {
        return self.module.get_function(&function_name).unwrap();
    }
}
