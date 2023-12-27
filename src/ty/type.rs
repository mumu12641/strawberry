use super::{primitive::PrimitiveType, class_type::ClassType};

pub enum Type{
    Primitive(PrimitiveType),
    ClassType(ClassType)
}