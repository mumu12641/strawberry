use core::num;
use std::ops::Deref;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, StructType},
    values::{AnyValue, BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace,
};

use crate::{
    ctx::CompileContext,
    parser::ast::class::{Class, Feature},
    utils::table::{ClassTable, SymbolTable, Tables},
    OBJECT,
};

use super::{
    env::{Env, VarEnv},
    types::LLVMType,
};
/// class prototype
/// class method table prototype
/// class init method
/// class constructor method
/// expressions
pub struct IrGenerator<'ctx> {
    // pub classes: Vec<Class>,
    // pub ctx: &'ctx Context,
    // pub module: Module<'ctx>,
    // pub builder: Builder<'ctx>,
    // pub class_table: &'ctx mut ClassTable,
    // pub tables: Tables,
    // pub env: Env<'ctx>,
    pub ctx: CompileContext,
    pub llvm_ctx: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub env: Env<'ctx>,
}

impl<'ctx> IrGenerator<'ctx> {
    pub fn new(
        ctx: CompileContext,
        llvm_ctx: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
    ) -> Self {
        IrGenerator {
            ctx,
            llvm_ctx,
            module,
            builder,
            env: Env::new(),
        }
    }

    pub unsafe fn ir_generate(&mut self) {
        //* for placeholder */
        self.gen_placeholders();

        //* generate method ir */
        self.gen_methods();

        //* generate main function */
        self.gen_main();

        let _ = self.module.print_to_file("./test.ll");
    }

    fn gen_constant(&self) {
        let mut i = 0;
        for s in &self.ctx.tables.string_table {
            self.builder
                .build_global_string_ptr(&s.as_str(), &format!("str_const_{}", i));

            i += 1;
        }
    }

    fn gen_placeholders(&mut self) {
        for class in &self.ctx.classes {
            self.env.struct_type_place_holders.insert(
                class.name.clone(),
                self.llvm_ctx.opaque_struct_type(&class.name),
            );
        }
        //* for methods place holder*/
        self.gen_method_placeholder();

        //* generate class prototypes */
        //* methods placeholder */
        let classes = self.ctx.classes.clone();
        for class in classes {
            self.env
                .var_env
                .insert(class.name.clone(), SymbolTable::new());
            self.env.var_env.get_mut(&class.name).unwrap().enter_scope();
            class.emit_llvm_type(self);
        }

        //* for inkwell's bug */
        //* for string const placeholder */
        let function = self.module.add_function(
            "_placeholder",
            self.llvm_ctx.void_type().fn_type(&[], false),
            None,
        );
        let entry_block = self.llvm_ctx.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        //* for string constant */
        self.gen_constant();

        self.builder.build_return(None);
    }

    fn gen_main(&self) {
        let main_function =
            self.module
                .add_function("main", self.llvm_ctx.i32_type().fn_type(&[], false), None);
        let main_entry_block = self.llvm_ctx.append_basic_block(main_function, "entry");
        let zero = self.llvm_ctx.i32_type().const_int(0, false);
        self.builder.position_at_end(main_entry_block);

        let main_ptr = self
            .builder
            .build_malloc(self.module.get_struct_type("Main").unwrap(), "main_ptr");
        self.builder.build_call(
            self.module.get_function("Main.init").unwrap(),
            &[main_ptr.unwrap().into()],
            "main",
        );
        let result = self.builder.build_call(
            self.module.get_function("Main.main").unwrap(),
            &[],
            "main_result",
        );

        self.builder
            .build_return(Some(&result.try_as_basic_value().left().unwrap()));
    }

    fn gen_method_placeholder(&self) {
        //* for method place holder */
        for class in &self.ctx.classes {
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
                        self.get_funtion_type(
                            params.as_slice(),
                            Some(
                                self.get_llvm_type(LLVMType::from_string_to_llvm_type(
                                    &method.return_type,
                                ))
                                .into(),
                            ),
                        ),
                        None,
                    );
                }
            }
        }
    }

    fn gen_methods(&mut self) {
        //* emit init method */
        let classes = self.ctx.classes.clone();
        for class in &classes {
            let init_method = self.get_function(format!("{}.init", &class.name));
            let entry_block = self.llvm_ctx.append_basic_block(init_method, "entry");
            self.builder.position_at_end(entry_block);

            //* call parents' init method */
            if &class.name != OBJECT {
                let parent = self.ctx.class_table.get_parent(&class.name);
                let cast = self.builder.build_bitcast(
                    init_method.get_first_param().unwrap(),
                    self.get_llvm_type(LLVMType::from_string_to_llvm_type(&parent)),
                    "cast",
                );
                self.builder.build_call(
                    self.get_function(format!("{}.init", parent)),
                    &[cast.into()],
                    "a",
                );
            }
            //* store method table to class */
            let m = self.builder.build_struct_gep(
                init_method.get_first_param().unwrap().into_pointer_value(),
                1,
                "method_table",
            );
            self.builder.build_store(
                m.unwrap(),
                self.module
                    .get_global(&format!("{}_dispatch_table", &class.name))
                    .unwrap(),
            );

            let mut num = 0;
            for f in &class.features {
                match f {
                    Feature::Attribute(attr) => {
                        if let Some(e) = attr.init.deref() {
                            let val = e.emit_llvm_ir(self);
                            let off = self
                                .env
                                .var_env
                                .get(&class.name.clone())
                                .unwrap()
                                .find(&attr.name)
                                .unwrap()
                                .into_offset();

                            let ptr = self
                                .builder
                                .build_struct_gep(
                                    init_method.get_first_param().unwrap().into_pointer_value(),
                                    off,
                                    &format!("val_{}", num),
                                )
                                .unwrap();
                            num += 1;
                            self.builder.build_store(ptr, val);
                        }
                    }
                    _ => {}
                }
            }

            self.builder.build_return(None);
        }

        //* emit methods */
        for class in &self.ctx.classes {
            //* curr class */
            self.env.curr_class = class.name.clone();

            for f in &class.features {
                if let Feature::Method(method) = f {
                    self.env.var_env.get_mut(&class.name).unwrap().enter_scope();

                    let m = self
                        .module
                        .get_function(&format!("{}.{}", &class.name, &method.name))
                        .unwrap();
                    self.env.curr_function = Some(m);

                    for p in method.param.deref().iter().enumerate() {
                        self.env.var_env.get_mut(&class.name).unwrap().add(
                            &p.1 .0,
                            &VarEnv::Value(m.get_nth_param(p.0.try_into().unwrap()).unwrap()),
                        );
                    }
                    let entry_block = self
                        .llvm_ctx
                        .append_basic_block(self.env.curr_function.unwrap(), "entry");

                    self.env.curr_block = Some(entry_block);
                    self.builder.position_at_end(self.env.curr_block.unwrap());

                    if let Some(exps) = method.body.deref() {
                        for e in exps {
                            e.emit_llvm_ir(&self);
                        }
                    }

                    self.env.var_env.get_mut(&class.name).unwrap().exit_scope();
                }
            }
        }
    }

    pub fn get_llvm_type(&self, llvm_type: LLVMType) -> BasicTypeEnum<'ctx> {
        match llvm_type {
            LLVMType::I32 => BasicTypeEnum::IntType(self.llvm_ctx.i32_type()),
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
            None => self.llvm_ctx.void_type().fn_type(params, false),
        }
    }
    pub fn get_function(&self, function_name: String) -> FunctionValue<'ctx> {
        return self.module.get_function(&function_name).unwrap();
    }
}
