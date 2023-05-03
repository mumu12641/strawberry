use std::{
    collections::{btree_map::Values, HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::grammar::ast::class::{Class, Feature, MethodDecl, VarDecl};

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
                line_num: 0, // features: vec![],
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
                line_num: 0,
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
                line_num: 0,
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
                line_num: 0,
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
        println!("Print Symbol Table");
        for i in self.scopes.iter().rev() {
            for j in &i.type_map {
                println!("key -> {}    value -> {}", j.0, j.1);
            }
        }
        println!();
    }
}

#[derive(Debug)]
pub struct Scope<K: PartialEq + Eq + Hash + Display, V: PartialEq + Eq + Display> {
    pub type_map: HashMap<K, V>,
}

impl<K: PartialEq + Eq + Hash + Clone + Display, V: PartialEq + Eq + Clone + Display> Scope<K, V> {
    pub fn add(&mut self, k: &K, v: &V) {
        self.type_map.insert(k.clone(), v.clone());
    }
}
