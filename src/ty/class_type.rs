use std::collections::HashMap;

use crate::parser::ast::{class::Class, TypeName};

use super::r#type::Type;

#[derive(Default, Debug, Clone)]
pub struct ClassType {
    pub name: TypeName,
    pub fields: Vec<Type>,
    pub methods: Vec<Type>,
}
impl PartialEq for ClassType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields && self.methods == other.methods
    }
}

impl Eq for ClassType {}
