use std::collections::{HashMap, HashSet};

use crate::ast::{Class, Feature, MethodDecl, ParamDecl, VarDecl};

#[derive(Debug)]
pub struct Tables {
    pub string_table: HashSet<String>,
    pub int_table: HashSet<String>,
    pub id_table: HashSet<String>,
}
impl Tables {
    pub fn new() -> Tables {
        Tables {
            string_table: HashSet::new(),
            int_table: HashSet::new(),
            id_table: HashSet::new(),
        }
    }
}

pub struct ClassTable {
    pub classes: HashMap<String, Class>,
    pub inheritance: HashMap<String, Vec<Class>>,
}

impl ClassTable {
    pub fn new() -> ClassTable {
        ClassTable {
            classes: HashMap::new(),
            inheritance: HashMap::new(),
        }
    }

    pub fn install_basic_class(&mut self) -> bool {
        // install basic classes
        let string = "String".to_string();
        let object = "Object".to_string();
        let int = "Int".to_string();
        let bool = "Bool".to_string();

        self.classes.insert(
            object.clone(),
            Class {
                name: object.clone(),
                parent: Some("None".to_string()),
                features: vec![Feature::Method(MethodDecl {
                    name: "print".to_string(),
                    param: Box::new(vec![("s".to_string(), string.clone())]),
                    return_type: object.clone(),
                    body: Box::new(None),
                })],
            },
        );
        self.classes.insert(
            string.clone(),
            Class {
                name: string.clone(),
                parent: Some(object.clone()),
                features: vec![Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: "prim_slot".to_string(),
                    init: Box::new(None),
                })],
            },
        );
        self.classes.insert(
            int.clone(),
            Class {
                name: int.clone(),
                parent: Some(object.clone()),
                features: vec![Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: "prim_slot".to_string(),
                    init: Box::new(None),
                })],
            },
        );
        self.classes.insert(
            bool.clone(),
            Class {
                name: bool.clone(),
                parent: Some(object.clone()),
                features: vec![Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: "prim_slot".to_string(),
                    init: Box::new(None),
                })],
            },
        );

        // Option
        match self.classes.get(&object.clone()) {
            Some(c) => {
                self.inheritance.insert(string.clone(), vec![c.clone()]);
                self.inheritance.insert(int.clone(), vec![c.clone()]);
                self.inheritance.insert(bool.clone(), vec![c.clone()]);
            }
            None => {}
        }

        true
    }
}
