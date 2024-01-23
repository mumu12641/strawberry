use crate::{
    parser::ast::{class, Type},
    BOOL, INT, VOID,
};

use inkwell::{
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType},
    values::{BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace,
};

use crate::parser::ast::class::{Class, Feature};

use super::ir::{self, IrGenerator};
pub enum LLVMType {
    I32,
    Bool,
    // Void,
    StructType { type_: Type },
}

impl LLVMType {
    pub fn from_string_to_llvm_type(type_name: &String) -> Self {
        if type_name == INT {
            return LLVMType::I32;
        } else if type_name == BOOL {
            return LLVMType::Bool;
        } else {
            return LLVMType::StructType {
                type_: type_name.to_string(),
            };
        }
    }
    pub fn is_void_type(type_name: &String) -> bool {
        if type_name == VOID {
            return true;
        }
        return false;
    }
}
