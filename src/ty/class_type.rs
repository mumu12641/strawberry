use std::collections::HashMap;

use super::r#type::Type;

pub struct ClassType {
    pub name: String,
    pub fields: HashMap<String, Box<Type>>,
    pub methods: HashMap<String, Box<Type>>,
}
