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

        let object_ = Class {
            name: OBJECT.to_string(),
            parent: Some("None".to_string()),
            features: vec![
                Feature::Method(MethodDecl {
                    name: "print".to_string(),
                    param: Box::new(vec![("val".to_string(), STRING.to_string())]),
                    return_type: OBJECT.to_string(),
                    body: Box::new(None),
                    position: EMPTY,
                }),
                Feature::Method(MethodDecl {
                    name: "to_string".to_string(),
                    param: Box::new(vec![]),
                    return_type: STRING.to_string(),
                    body: Box::new(None),
                    position: EMPTY,
                }),
                Feature::Method(MethodDecl {
                    name: "malloc".to_string(),
                    param: Box::new(vec![("object".to_string(), OBJECT.to_string())]),
                    return_type: OBJECT.to_string(),
                    body: Box::new(None),
                    position: EMPTY,
                }),
            ],
            position: (0, 0), // features: vec![],
            file_name: OBJECT.to_string(),
        };
        let string_ = Class {
            name: STRING.to_string(),
            parent: Some(OBJECT.to_string()),
            features: vec![
                Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: Some("prim_slot".to_string()),
                    init: Box::new(None),
                    position: EMPTY,
                }),
                Feature::Attribute(VarDecl {
                    name: "len".to_string(),
                    type_: Some("prim_slot".to_string()),
                    init: Box::new(None),
                    position: EMPTY,
                }),
                Feature::Method(MethodDecl {
                    name: "concat".to_string(),
                    param: Box::new(vec![
                        ("dest".to_string(), STRING.to_string()),
                        ("src".to_string(), STRING.to_string()),
                    ]),
                    return_type: STRING.to_string(),
                    body: Box::new(None),
                    position: EMPTY,
                }),
            ],
            position: (0, 0),
            file_name: STRING.to_string(),
        };
        let int_ = Class {
            name: INT.to_string(),
            parent: Some(OBJECT.to_string()),
            features: vec![
                Feature::Attribute(VarDecl {
                    name: "val".to_string(),
                    type_: Some("prim_slot".to_string()),
                    init: Box::new(None),
                    position: EMPTY,
                }),
                Feature::Method(MethodDecl {
                    name: "to_string".to_string(),
                    param: Box::new(vec![]),
                    return_type: STRING.to_string(),
                    body: Box::new(None),
                    position: EMPTY,
                }),
            ],
            position: (0, 0),
            file_name: INT.to_string(),
        };
        let bool_ = Class {
            name: BOOL.to_string(),
            parent: Some(OBJECT.to_string()),
            features: vec![Feature::Attribute(VarDecl {
                name: "val".to_string(),
                type_: Some("prim_slot".to_string()),
                init: Box::new(None),
                position: EMPTY,
            })],
            position: (0, 0),
            file_name: BOOL.to_string(),
        };

        self.classes.insert(OBJECT.to_string(), object_.clone());
        self.classes.insert(STRING.to_string(), string_.clone());
        self.classes.insert(INT.to_string(), int_.clone());
        self.classes.insert(BOOL.to_string(), bool_.clone());

        // Option
        match self.classes.get(&OBJECT.to_string()) {
            Some(c) => {
                self.inheritance
                    .insert(STRING.to_string(), vec![c.clone(), string_.clone()]);
                self.inheritance
                    .insert(INT.to_string(), vec![c.clone(), int_.clone()]);
                self.inheritance
                    .insert(BOOL.to_string(), vec![c.clone(), bool_.clone()]);
                self.inheritance.insert(OBJECT.to_string(), vec![c.clone()]);
            }
            None => {}
        }
        // self.inheritance.insert(OBJECT.to_string(), vec![])

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

    pub fn get_parent(&mut self, child: &Type) -> String {
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
pub struct Scope<K: PartialEq + Eq + Hash + Display, V: PartialEq + Eq + Display> {
    pub type_map: HashMap<K, V>,
}

impl<K: PartialEq + Eq + Hash + Clone + Display, V: PartialEq + Eq + Clone + Display> Scope<K, V> {
    pub fn add(&mut self, k: &K, v: &V) {
        self.type_map.insert(k.clone(), v.clone());
    }
}
