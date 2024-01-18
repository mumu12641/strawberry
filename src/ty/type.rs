use std::ops::Deref;

use super::{class_type::ClassType, method_type::MethodType, primitive::PrimitiveType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Primitive(PrimitiveType),
    ClassType(ClassType),
    MethodType(MethodType),
    // DeclType(DeclType),
    Slot,
}

impl Type {
    pub fn as_primitive_type(&self) -> PrimitiveType {
        if let Type::Primitive(t) = self {
            t.clone()
        } else {
            panic!("error!")
        }
    }
    pub fn as_method_type(&self) -> MethodType {
        if let Type::MethodType(t) = self {
            t.clone()
        } else {
            panic!("error!")
        }
    }
    pub fn as_class_type(&self) -> ClassType {
        if let Type::ClassType(t) = self {
            t.clone()
        } else {
            panic!("error!")
        }
    }
    // pub fn as_decl_type(&self) -> DeclType {
    //     if let Type::DeclType(t) = self {
    //         t.clone()
    //     } else {
    //         panic!("error!")
    //     }
    // }
}
