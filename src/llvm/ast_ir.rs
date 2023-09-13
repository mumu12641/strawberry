use crate::{
    grammar::ast::{
        class,
        expr::{Expr, Self_},
        Type,
    },
    llvm::ir,
    INT,
};

use inkwell::{
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType},
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace,
};

use crate::grammar::ast::class::{Class, Feature};

use super::{ir::IrGenerator, types::LLVMType};

impl Expr {
    pub fn emit_llvm_ir<'a>(&self, ir_genrator: &'a IrGenerator) -> BasicValueEnum<'a> {
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
            _ => {}
        }
        unreachable!()
    }
}
// impl EmitLLVMIR<'_> for Expr {
//     fn emit_llvm_ir<V: BasicValue<'a>>(&self, ir_genrator: &mut IrGenerator) -> V {
//         todo!()
//     }
// }
// impl EmitLLVMIR for  {

// }

impl Class {
    pub fn emit_llvm_type<'a>(
        &self,
        ir_genrator: &'a mut IrGenerator,
    ) {
        //* class prototype */
        //*     NULL flag
        //*     _dispatch_table
        //*     attrs

        let method_prototype: BasicTypeEnum<'_> = self.emit_method_table_llvm_type(ir_genrator);
        let placeholder = ir_genrator
            .env
            .struct_type_place_holders
            .get(&self.name)
            .unwrap();
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
                    ir_genrator
                        .env
                        .field_offset_map
                        .insert((self.name.clone(), attr.name.clone()), offset);
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
            .ctx
            .opaque_struct_type(&format!("{}_dispatch_table_prototype", &self.name));
        let mut methods: Vec<BasicTypeEnum> = vec![];
        let mut method_names: Vec<String> = vec![format!("{}.init", &self.name)];
        for f in &self.features {
            if let Feature::Method(method) = f {
                let mut params_type: Vec<BasicMetadataTypeEnum> =
                    vec![BasicMetadataTypeEnum::StructType(
                        ir_genrator.module.get_struct_type(&self.name).unwrap(),
                    )];
                for param in method.param.as_ref() {
                    params_type.push(
                        ir_genrator
                            .get_llvm_type(LLVMType::from_string_to_llvm_type(&param.1))
                            .into(),
                    )
                }
                method_names.push(format!("{}.{}", &self.name, method.name));
                methods.push(BasicTypeEnum::PointerType(
                    ir_genrator
                        .ctx
                        .void_type()
                        .fn_type(params_type.as_slice(), false)
                        .ptr_type(AddressSpace::default()),
                ));
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
