use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::parser::ast::{
    class::{Class, ConstructorDecl, Feature},
    Type,
};

#[derive(Debug, Clone)]
pub struct Tables {
    pub string_table: Vec<String>,
    pub int_table: HashSet<String>,
    pub id_table: HashSet<String>,
}
impl Tables {
    pub fn new() -> Tables {
        Tables {
            string_table: Vec::new(),
            int_table: HashSet::new(),
            id_table: HashSet::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassTable {
    pub classes: HashMap<Type, Class>,
    pub inheritance: HashMap<Type, Vec<Class>>,
    pub class_constructors: HashMap<Type, Vec<ConstructorDecl>>,
}

impl ClassTable {
    pub fn new() -> ClassTable {
        ClassTable {
            classes: HashMap::new(),
            inheritance: HashMap::new(),
            class_constructors: HashMap::new(),
        }
    }
    pub fn get_classes(&mut self) -> HashMap<String, Class> {
        return self.classes.clone();
    }

    pub fn get_inheritance(&self) -> HashMap<String, Vec<Class>> {
        return self.inheritance.clone();
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

    pub fn get_parent(&self, child: &Type) -> String {
        let binding = self.get_inheritance();
        let v = binding.get(child).unwrap();
        v.get(v.len() - 2).unwrap().name.clone()
    }

    pub fn get_attr_num_recursive(&mut self, child: &Type) -> usize {
        let mut attr_num = 0;
        for curr_class in self.inheritance.get(child).unwrap() {
            for attr_ in &curr_class.features {
                if let Feature::Attribute(_) = attr_ {
                    attr_num += 1;
                }
            }
        }
        return attr_num;
    }
}

pub struct SymbolTable<K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Clone> {
    pub scopes: Vec<Scope<K, V>>,
}

impl<K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Clone> SymbolTable<K, V> {
    pub fn new() -> SymbolTable<K, V> {
        SymbolTable { scopes: Vec::new() }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope {
            type_map: HashMap::new(),
        });
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

    // pub fn debug(&mut self) {
    //     for i in self.scopes.iter().rev() {
    //         for j in &i.type_map {
    //             println!("key -> {}    value -> {}", j.0, j.1);
    //         }
    //     }
    //     println!();
    // }
}

#[derive(Debug, Clone)]
pub struct Scope<K: PartialEq + Eq + Hash, V: PartialEq + Eq> {
    pub type_map: HashMap<K, V>,
}

impl<K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Clone> Scope<K, V> {
    pub fn add(&mut self, k: &K, v: &V) {
        self.type_map.insert(k.clone(), v.clone());
    }
}
