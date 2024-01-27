use core::num;
use std::{
    ops::{Add, Deref},
    vec,
};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, StructType},
    values::{AnyValue, BasicValueEnum, FunctionValue, IntValue, PointerValue},
    AddressSpace,
};

use crate::{
    ctx::CompileContext,
    parser::ast::{
        class::{Class, Feature},
        expr::TypeGet,
        Type,
    },
    utils::table::{ClassTable, SymbolTable, Tables},
    OBJECT, STR,
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
    // pub env: Env<'ctx>,
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
            // env: Env::new(),
        }
    }

    pub unsafe fn ir_generate(&mut self, env: &mut Env<'ctx>) {
        //* for placeholder */
        self.gen_placeholders(env);

        //* generate method ir */
        self.gen_methods(env);

        //* generate main function */
        self.gen_main();

        let _ = self.module.print_to_file("./test.ll");
    }

    fn gen_constant(&self) {
        //* string const */
        let mut i = 0;
        for s in &self.ctx.tables.string_table {
            let len = s.len() + 1;
            let const_name = format!("str_const_{}", i);
            self.builder
                .build_global_string_ptr(&s.as_str(), &const_name);

            // ! string const?
            // let val: Vec<BasicValueEnum> = vec![
            //     BasicValueEnum::IntValue(self.llvm_ctx.i32_type().const_int(1, false)),
            //     BasicValueEnum::PointerValue(
            //         self.module
            //             .get_global("String_dispatch_table")
            //             .unwrap()
            //             .as_pointer_value(),
            //     ),
            //     BasicValueEnum::IntValue(self.llvm_ctx.i32_type().const_int(len.try_into().unwrap(), false)),
            //     BasicValueEnum::PointerValue(
            //         self.module.get_global(&const_name).unwrap().as_pointer_value(),
            //     ),
            // ];
            // let string_prototype = self.llvm_ctx.get_struct_type("String").unwrap();
            // let init = string_prototype.const_named_struct(&val);
            // self.module
            //     .add_global(
            //         string_prototype,
            //         Some(AddressSpace::default()),
            //         &format!("string_const_{}", i),
            //     )
            //     .set_initializer(&init);

            i += 1;
        }

        //* bool const */
        let bool_protype = self.llvm_ctx.get_struct_type("Bool").unwrap();
        let global_dispatch = self
            .module
            .get_global("Bool_dispatch_table")
            .unwrap()
            .as_pointer_value();
        let true_val: Vec<BasicValueEnum> = vec![
            BasicValueEnum::IntValue(self.llvm_ctx.i32_type().const_int(1, false)),
            BasicValueEnum::PointerValue(global_dispatch),
            BasicValueEnum::IntValue(self.llvm_ctx.i32_type().const_int(1, false)),
        ];
        let false_val: Vec<BasicValueEnum> = vec![
            BasicValueEnum::IntValue(self.llvm_ctx.i32_type().const_int(1, false)),
            BasicValueEnum::PointerValue(global_dispatch),
            BasicValueEnum::IntValue(self.llvm_ctx.i32_type().const_int(0, false)),
        ];
        let true_init = bool_protype.const_named_struct(true_val.as_slice());
        let false_init = bool_protype.const_named_struct(false_val.as_slice());
        self.module
            .add_global(bool_protype, Some(AddressSpace::default()), "bool_const_0")
            .set_initializer(&false_init);
        self.module
            .add_global(bool_protype, Some(AddressSpace::default()), "bool_const_1")
            .set_initializer(&true_init);
    }

    fn gen_placeholders(&mut self, env: &mut Env<'ctx>) {
        for class in &self.ctx.classes {
            env.struct_type_place_holders.insert(
                class.name.clone(),
                self.llvm_ctx.opaque_struct_type(&class.name.clone()),
            );
        }
        //* for methods place holder*/
        self.gen_method_placeholder();

        //* generate class prototypes */
        //* methods placeholder */
        let classes = self.ctx.classes.clone();
        for class in classes {
            env.var_env.insert(class.name.clone(), SymbolTable::new());
            env.var_env.get_mut(&class.name).unwrap().enter_scope();
            class.emit_llvm_type(self, env);
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
            &[main_ptr.unwrap().into()],
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
                            self.get_return_type(&method.return_type),
                        ),
                        None,
                    );
                }
            }
        }
    }

    fn gen_methods(&mut self, env: &mut Env<'ctx>) {
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
                            let off = env
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

                            if (e.get_type() == STR) {
                                unsafe {
                                    let raw_val = e.emit_llvm_ir(self, env);
                                    let val = self.builder.build_in_bounds_gep(
                                        raw_val.into_pointer_value(),
                                        &[
                                            self.llvm_ctx.i32_type().const_int(0, false),
                                            self.llvm_ctx.i32_type().const_int(0, false),
                                        ],
                                        "val",
                                    );
                                    self.builder.build_store(ptr, val);
                                };
                            } else {
                                let raw_val = e.emit_llvm_ir(self, env);
                                self.builder.build_store(ptr, raw_val);
                            }
                            num += 1;
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
            env.curr_class = class.name.clone();

            for f in &class.features {
                if let Feature::Method(method) = f {
                    env.var_env.get_mut(&class.name).unwrap().enter_scope();

                    let m = self
                        .module
                        .get_function(&format!("{}.{}", &class.name, &method.name))
                        .unwrap();
                    env.curr_function = Some(m);

                    for p in method.param.deref().iter().enumerate() {
                        env.var_env.get_mut(&class.name).unwrap().add(
                            &p.1 .0,
                            &VarEnv::Value(m.get_nth_param(p.0.try_into().unwrap()).unwrap()),
                        );
                    }
                    let entry_block = self
                        .llvm_ctx
                        .append_basic_block(env.curr_function.unwrap(), "entry");

                    env.curr_block = Some(entry_block);
                    self.builder.position_at_end(env.curr_block.unwrap());

                    if let Some(exps) = method.body.deref() {
                        for e in exps {
                            e.emit_llvm_ir(self, env);
                        }
                    }

                    env.var_env.get_mut(&class.name).unwrap().exit_scope();
                }
            }
        }
    }

    pub fn get_llvm_type(&self, llvm_type: LLVMType) -> BasicTypeEnum<'ctx> {
        match llvm_type {
            LLVMType::I32 => BasicTypeEnum::IntType(self.llvm_ctx.i32_type()),
            // LLVMType::Bool => BasicTypeEnum::IntType(self.llvm_ctx.i32_type()),
            LLVMType::Str => BasicTypeEnum::PointerType(
                self.llvm_ctx.i8_type().ptr_type(AddressSpace::default()),
            ),
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

    pub fn get_return_type(&self, llvm_type: &Type) -> Option<BasicMetadataTypeEnum<'ctx>> {
        return if LLVMType::is_void_type(llvm_type) {
            None
        } else {
            Some(
                self.get_llvm_type(LLVMType::from_string_to_llvm_type(llvm_type))
                    .into(),
            )
        };
    }

    pub fn get_funtion_type(
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
