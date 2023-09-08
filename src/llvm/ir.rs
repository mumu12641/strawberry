use std::{collections::HashMap, fmt::format, ops::Deref};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, StructType},
};

use crate::{
    grammar::ast::{
        class::{self, Class, Feature},
        Type,
    },
    utils::table::Tables,
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
}

impl<'ctx> IrGenerator<'ctx> {
    pub fn ir_generate(&self) {
        //* generate class prototypes */
        //* for placeholder */
        let mut placeholder: HashMap<&Type, StructType> = HashMap::new();
        for class in &self.classes {
            placeholder.insert(&class.name, self.ctx.opaque_struct_type(&class.name));
        }
        for class in &self.classes {
            class.emit_llvm_type(self, *placeholder.get(&class.name).unwrap());
        }

        //* generate method ir */
        //* first is init_method*/
        self.gen_init_method();
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
        for class in &self.classes {
            let init_method = self.module.add_function(
                &format!("{}.init", &class.name),
                self.get_funtion_type(
                    &[self
                        .get_llvm_type(LLVMType::from_string_to_llvm_type(&class.name))
                        .into()],
                    None,
                ),
                None,
            );
            let entry_block = self.ctx.append_basic_block(init_method, "entry");
            self.builder.position_at_end(entry_block);
            self.builder.build_return(None);
        }
    }

    fn gen_methods(&self) {
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
                    let method_proto = self.module.add_function(
                        &format!("{}.{}", &class.name, method.name),
                        // LLVMType::from_string_to_llvm_type(method.return_type),
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
                    return BasicTypeEnum::StructType(t);
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
                _ => unreachable!(),
            },
            None => self.ctx.void_type().fn_type(params, false),
        }
    }
}
