use std::ops::Deref;

use crate::parser::ast::{
    class,
    expr::{Dispatch, DispatchExpr, Expr, IdentifierSrtuct, Let, Return, Self_},
    Type,
};

use inkwell::{
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType},
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace,
};

use crate::parser::ast::class::{Class, Feature};

use super::{
    env::{Env, VarEnv},
    ir::{self, IrGenerator},
    types::LLVMType,
};

impl Expr {
    pub fn emit_llvm_ir<'a>(
        &self,
        ir_genrator: &'a IrGenerator,
        env: &'a mut Env,
    ) -> BasicValueEnum<'a> {
        match self {
            Expr::Int(e) => {
                // return ir_genrator.builder.
                return inkwell::values::BasicValueEnum::IntValue(
                    ir_genrator
                        .get_llvm_type(LLVMType::I32)
                        .into_int_type()
                        .const_int(*e, false),
                );
            }
            Expr::Bool(bool_const) => {
                let index = if *bool_const { 1 } else { 0 };
                return ir_genrator
                    .module
                    .get_global(&format!("bool_const_{}", index))
                    .unwrap()
                    .as_basic_value_enum();
            }
            Expr::Str(str_const) => {
                let index = ir_genrator
                    .ctx
                    .tables
                    .string_table
                    .iter()
                    .position(|x| x == str_const)
                    .unwrap();
                return ir_genrator
                    .module
                    .get_global(&format!("str_const_{}", index))
                    .unwrap()
                    .as_basic_value_enum();
            }
            Expr::Dispatch(e) => {
                return e.emit_llvm_ir(ir_genrator, env);
            }
            Expr::Identifier(e) => {
                return e.emit_llvm_ir(ir_genrator, env);
            }
            Expr::Let(e) => {
                return e.emit_llvm_ir(ir_genrator, env);
            }
            Expr::Self_(_) => {
                return env.curr_function.unwrap().get_first_param().unwrap();
            }

            Expr::Return(e) => {
                return e.emit_llvm_ir(ir_genrator, env);
            }
            _ => {}
        };
        ir_genrator.get_llvm_type(LLVMType::I32).const_zero()
    }
}

impl IdentifierSrtuct {
    pub fn emit_llvm_ir<'a>(
        &self,
        ir_genrator: &'a IrGenerator,
        env: &'a mut Env,
    ) -> BasicValueEnum<'a> {
        let map = &env.var_env;
        // e.name
        let symbol_table = map.get(&env.curr_class).unwrap();
        let var = symbol_table.find(&self.name).unwrap();
        match var {
            super::env::VarEnv::Field(off) => ir_genrator.get_llvm_type(LLVMType::I32).const_zero(),
            super::env::VarEnv::Value(val) => return val.clone(),
        }
    }
}

impl Let {
    pub fn emit_llvm_ir<'a>(
        &self,
        ir_genrator: &'a IrGenerator,
        env: &'a mut Env,
    ) -> BasicValueEnum<'a> {
        // self.var_decls
        for decl in self.var_decls.deref() {
            match decl.init.deref() {
                Some(e) => {
                    let val = e.emit_llvm_ir(ir_genrator, env);
                    env.get_curr_env().add(&decl.name, &VarEnv::Value(val));
                }
                None => todo!(),
            }
        }
        ir_genrator.get_llvm_type(LLVMType::I32).const_zero()
    }
}

impl Dispatch {
    pub fn emit_llvm_ir<'a>(
        &self,
        ir_genrator: &'a IrGenerator,
        env: &'a mut Env,
    ) -> BasicValueEnum<'a> {
        match &self.expr {
            DispatchExpr::Field(field) => {
                let off = env
                    .var_env
                    .get(&env.curr_class)
                    .unwrap()
                    .find(field)
                    .unwrap()
                    .into_offset();

                let target = self.target.emit_llvm_ir(ir_genrator, env);
                let result = ir_genrator
                    .builder
                    .build_struct_gep(target.into_pointer_value(), off, "a")
                    .unwrap();
                let b = ir_genrator.builder.build_load(result, "b");
                return b.into();
            }
            DispatchExpr::Method(_) => {}
        }
        unimplemented!()
    }
}

impl Return {
    pub fn emit_llvm_ir<'a>(
        &self,
        ir_genrator: &'a IrGenerator,
        env: &'a mut Env,
    ) -> BasicValueEnum<'a> {
        if let Some(e) = self.val.as_deref() {
            ir_genrator
                .builder
                .build_return(Some(&e.emit_llvm_ir(ir_genrator, env)));
        } else {
            ir_genrator.builder.build_return(None);
        }
        ir_genrator.get_llvm_type(LLVMType::I32).const_zero()
    }
}

impl Class {
    pub fn emit_llvm_type<'a>(&self, ir_genrator: &'a mut IrGenerator, env: &'a mut Env) {
        //* class prototype */
        //*     NULL flag
        //*     _dispatch_table
        //*     attrs

        let method_prototype: BasicTypeEnum<'_> = self.emit_method_table_llvm_type(ir_genrator);
        let placeholder = env.struct_type_place_holders.get(&self.name).unwrap();
        let mut attrs: Vec<BasicTypeEnum> = vec![
            ir_genrator.get_llvm_type(LLVMType::I32),
            BasicTypeEnum::PointerType(method_prototype.ptr_type(AddressSpace::default())),
        ];

        for f in &self.features {
            match f {
                Feature::Attribute(attr) => {
                    attrs.push(
                        ir_genrator.get_llvm_type(LLVMType::from_string_to_llvm_type(
                            &attr.type_.clone().unwrap(),
                        )),
                    );
                }
                _ => {}
            }
        }
        placeholder.set_body(attrs.as_slice(), false);

        let mut offset = 2;
        for f in &self.features {
            match f {
                Feature::Attribute(attr) => {
                    env.field_offset_map
                        .insert((self.name.clone(), attr.name.clone()), offset);

                    env.var_env
                        .get_mut(&self.name)
                        .unwrap()
                        .add(&attr.name, &super::env::VarEnv::Field(offset));
                    offset += 1;
                }

                _ => {}
            }
        }
    }

    //* emit metthod table prototype */
    //* emit method table globale value */
    pub fn emit_method_table_llvm_type<'a>(
        &self,
        ir_genrator: &'a IrGenerator,
    ) -> BasicTypeEnum<'a> {
        let method_prototype = ir_genrator
            .llvm_ctx
            .opaque_struct_type(&format!("{}_dispatch_table_prototype", &self.name));
        let self_ptr = BasicMetadataTypeEnum::PointerType(
            ir_genrator
                .module
                .get_struct_type(&self.name)
                .unwrap()
                .ptr_type(AddressSpace::default()),
        );
        let mut methods: Vec<BasicTypeEnum> = vec![inkwell::types::BasicTypeEnum::PointerType(
            ir_genrator
                .llvm_ctx
                .void_type()
                .fn_type(&[self_ptr.clone()], false)
                .ptr_type(AddressSpace::default()),
        )];
        let mut method_names: Vec<String> = vec![format!("{}.init", &self.name)];
        for f in &self.features {
            if let Feature::Method(method) = f {
                let mut params_type: Vec<BasicMetadataTypeEnum> = vec![self_ptr.clone()];

                for param in method.param.as_ref() {
                    params_type.push(
                        ir_genrator
                            .get_llvm_type(LLVMType::from_string_to_llvm_type(&param.1))
                            .into(),
                    )
                }
                method_names.push(format!("{}.{}", &self.name, method.name));
                methods.push(
                    ir_genrator
                        .get_funtion_type(
                            params_type.as_slice(),
                            ir_genrator.get_return_type(&method.return_type),
                        )
                        .ptr_type(AddressSpace::default())
                        .into(),
                );
            }
        }
        method_prototype.set_body(methods.as_slice(), false);
        let method_ptr_values: Vec<BasicValueEnum> = ir_genrator
            .module
            .get_functions()
            .filter(|f| method_names.contains(&f.get_name().to_string_lossy().to_string()))
            .map(|f| BasicValueEnum::PointerValue(f.as_global_value().as_pointer_value()))
            .collect();
        let method_table_globale_value = ir_genrator
            .module
            .get_struct_type(&format!("{}_dispatch_table_prototype", &self.name))
            .unwrap()
            .const_named_struct(method_ptr_values.as_slice());
        ir_genrator
            .module
            .add_global(
                method_prototype,
                Some(AddressSpace::default()),
                &format!("{}_dispatch_table", &self.name),
            )
            .set_initializer(&method_table_globale_value);
        BasicTypeEnum::StructType(method_prototype)
    }
}
