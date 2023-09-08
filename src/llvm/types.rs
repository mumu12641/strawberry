use crate::{grammar::ast::Type, INT};

use inkwell::{
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType},
    AddressSpace,
};

use crate::grammar::ast::class::{Class, Feature};

use super::ir::{IrGenerator, self};
pub enum LLVMType {
    I32,

    StructType { type_: Type },
}

impl LLVMType {
    pub fn from_string_to_llvm_type(type_name: &String) -> Self {
        if type_name == INT {
            return LLVMType::I32;
        } else {
            return LLVMType::StructType {
                type_: type_name.to_string(),
            };
        }
    }
}
impl Class {
    pub fn emit_llvm_type<'a>(&'a self, ir_genrator: &'a IrGenerator, placeholder: StructType<'a>) {
        //* class prototype */
        //*     NULL flag
        //*     _dispatch_table
        //*     attrs

        let method_prototype: BasicTypeEnum<'_> = self.emit_method_table_llvm_type(ir_genrator);

        let mut attrs: Vec<BasicTypeEnum> = vec![
            ir_genrator.get_llvm_type(LLVMType::I32),
            BasicTypeEnum::PointerType(method_prototype.ptr_type(AddressSpace::default())),
        ];

        for f in &self.features {
            match f {
                Feature::Attribute(attr) => attrs.push(ir_genrator.get_llvm_type(
                    LLVMType::from_string_to_llvm_type(&attr.type_.clone().unwrap()),
                )),
                _ => {}
            }
        }
        placeholder.set_body(attrs.as_slice(), false);
    }
    pub fn emit_method_table_llvm_type<'a>(
        &'a self,
        ir_genrator: &'a IrGenerator,
    ) -> BasicTypeEnum {
        let method_prototype = ir_genrator
            .ctx
            .opaque_struct_type(&format!("{}_dispatch_table_prototype", &self.name));
        let mut methods: Vec<BasicTypeEnum> = vec![];
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
        let g = ir_genrator.module.add_global(
            method_prototype,
            Some(AddressSpace::default()),
            &format!("{}_dispatch_table", &self.name),
        );
        
        // ir_genrator.module.get_function(name)
        // g.set_initializer(value)
        BasicTypeEnum::StructType(method_prototype)
    }
}
