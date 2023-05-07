use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::{
    grammar::ast::{
        class::{Class, Feature, MethodDecl, VarDecl},
        Type,
    },
    BOOL, EMPTY, INT, OBJECT, STRING,
};

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
    pub fn get_classes(&mut self) -> HashMap<String, Class> {
        return self.classes.clone();
    }

    pub fn get_inheritance(&mut self) -> HashMap<String, Vec<Class>> {
        return self.inheritance.clone();
    }

    pub fn install_basic_class(&mut self) -> bool {
        // install basic classes

        self.classes.insert(
            OBJECT.to_string(),
            Class {
                name: OBJECT.to_string(),
                parent: Some("None".to_string()),
                features: vec![
                    Feature::Method(MethodDecl {
                        name: "print_str".to_string(),
                        param: Box::new(vec![("val".to_string(), STRING.to_string())]),
                        return_type: OBJECT.to_string(),
                        body: Box::new(None),
                        position: EMPTY,
                    }),
                    Feature::Method(MethodDecl {
                        name: "print_int".to_string(),
                        param: Box::new(vec![("val".to_string(), INT.to_string())]),
                        return_type: OBJECT.to_string(),
                        body: Box::new(None),
                        position: EMPTY,
                    }),
                ],
                position: (0, 0), // features: vec![],
                file_name: OBJECT.to_string(),
            },
        );
        self.classes.insert(
            STRING.to_string(),
            Class {
                name: STRING.to_string(),
                parent: Some(OBJECT.to_string()),
                features: vec![Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: "prim_slot".to_string(),
                    init: Box::new(None),
                    position: EMPTY,
                })],
                position: (0, 0),
                file_name: STRING.to_string(),
            },
        );
        self.classes.insert(
            INT.to_string(),
            Class {
                name: INT.to_string(),
                parent: Some(OBJECT.to_string()),
                features: vec![Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: "prim_slot".to_string(),
                    init: Box::new(None),
                    position: EMPTY,
                })],
                position: (0, 0),
                file_name: INT.to_string(),
            },
        );
        self.classes.insert(
            BOOL.to_string(),
            Class {
                name: BOOL.to_string(),
                parent: Some(OBJECT.to_string()),
                features: vec![Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: "prim_slot".to_string(),
                    init: Box::new(None),
                    position: EMPTY,
                })],
                position: (0, 0),
                file_name: BOOL.to_string(),
            },
        );

        // Option
        match self.classes.get(&OBJECT.to_string()) {
            Some(c) => {
                self.inheritance.insert(STRING.to_string(), vec![c.clone()]);
                self.inheritance.insert(INT.to_string(), vec![c.clone()]);
                self.inheritance.insert(BOOL.to_string(), vec![c.clone()]);
            }
            None => {}
        }

        true
    }

    pub fn is_less_or_equal(&self, child: &Type, parent: &Type) -> bool {
        if child == parent {
            return true;
        }
        if let Some(v) = self.inheritance.get(child) {
            for i in v {
                if &i.name == parent {
                    return true;
                }
            }
        }
        return false;
    }
}

pub struct SymbolTable<
    K: PartialEq + Eq + Hash + Clone + Display,
    V: PartialEq + Eq + Clone + Display,
> {
    pub scopes: Vec<Scope<K, V>>,
}

impl<K: PartialEq + Eq + Hash + Clone + Display, V: PartialEq + Eq + Clone + Display>
    SymbolTable<K, V>
{
    pub fn new() -> SymbolTable<K, V> {
        SymbolTable { scopes: Vec::new() }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope {
            type_map: HashMap::new(),
        })
    }

    pub fn exit_scope(&mut self) {
        self.debug();
        self.scopes.pop();
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        for i in self.scopes.iter().rev() {
            if let Some(v) = i.type_map.get(key) {
                return Some(v);
            }
        }
        return None;
    }

    pub fn add(&mut self, k: &K, v: &V) {
        if let Some(s) = self.scopes.pop() {
            let mut s = s;
            s.add(k, v);
            self.scopes.push(s);
        }
    }

    pub fn debug(&mut self) {
        println!();
        println!("before exit scope, Print Symbol Table");
        for i in self.scopes.iter().rev() {
            println!("***scope***");
            for j in &i.type_map {
                println!("key -> {}    value -> {}", j.0, j.1);
            }
        }
        println!();
    }
}

#[derive(Debug, Clone)]
pub struct Scope<K: PartialEq + Eq + Hash + Display, V: PartialEq + Eq + Display> {
    pub type_map: HashMap<K, V>,
}

impl<K: PartialEq + Eq + Hash + Clone + Display, V: PartialEq + Eq + Clone + Display> Scope<K, V> {
    pub fn add(&mut self, k: &K, v: &V) {
        self.type_map.insert(k.clone(), v.clone());
    }
}
