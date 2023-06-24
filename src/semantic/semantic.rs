use std::ops::DerefMut;

use crate::{
    // grammar::ast::{Class, Feature, MethodDecl},
    grammar::ast::{
        class::{Class, Feature},
        expr::{Expr, TypeChecker},
        Identifier, Type,
    },
    table::ClassTable,
    utils::table::SymbolTable,
    DEBUG,
    SELF,
};

/// * install constants and basic classes.
/// * get all classes not just user defined but also include IO, Object and so on.
/// * check Main
/// * check inherit
/// * check attributes
/// * check method override
/// * check all expressions
pub struct SemanticChecker {
    classes: Vec<Class>,
    symbol_table: SymbolTable<Identifier, Type>,
}

#[derive(Debug)]
pub struct SemanticError {
    pub err_msg: String,
}

impl SemanticChecker {
    pub fn new(classes_: Vec<Class>) -> SemanticChecker {
        SemanticChecker {
            classes: classes_,
            symbol_table: SymbolTable::new(),
        }
    }
    pub fn check(&mut self, class_table: &mut ClassTable) -> Result<Vec<Class>, SemanticError> {
        let mut main_flag = false;
        let mut main_method_flag = false;

        // check repeat class
        for i in &self.classes {
            if i.name == "Main".to_string() {
                main_flag = true;
                for feature in &i.features {
                    if let Feature::Method(m) = feature {
                        if m.name == "main".to_string() {
                            main_method_flag = true;
                        }
                    }
                }
            }
            if class_table.classes.contains_key(&i.name) {
                return Err(SemanticError {
                    err_msg: format!(
                        "{}:{}:{} ---> Class {} has been redefined!",
                        i.file_name, i.position.0, i.position.1, i.name
                    ),
                });
            } else {
                class_table.classes.insert(i.name.clone(), i.clone());
            }
        }

        // chech main
        if !main_flag {
            return Err(SemanticError {
                err_msg: format!("Your program is missing the Main class"),
            });
        }
        if !main_method_flag {
            return Err(SemanticError {
                err_msg: format!("Your program is missing the Main function"),
            });
        }

        // check inheritance
        for i in &self.classes {
            let mut inherit_vec: Vec<Class> = Vec::new();
            let mut curr_parent: Option<String>;
            inherit_vec.insert(0, i.clone());
            curr_parent = i.parent.clone();
            loop {
                match curr_parent {
                    Some(ref s) => {
                        if s.clone() == "None".to_string() {
                            // current is object
                            break;
                        } else if s == &i.name {
                            return Err(SemanticError {
                                err_msg: format!(
                                    "{}:{}:{} ---> There is an inheritance cycle about Class {}!",
                                    i.file_name, i.position.0, i.position.1, s
                                ),
                            });
                        } else {
                            if let Some(c) = class_table.classes.get(&(s.clone())) {
                                inherit_vec.insert(0, c.clone());
                                curr_parent = c.parent.clone();
                            } else {
                                return Err(SemanticError {
                                    err_msg: format!(
                                        "{}:{}:{} ---> Your Class {} inherits an undefined Class {} !",
                                        i.file_name, i.position.0, i.position.1,i.name,s
                                    ),
                                });
                            }
                        }
                    }
                    None => {
                        if let Some(c) = class_table.classes.get(&"Object".to_string()) {
                            inherit_vec.insert(0, c.clone());
                            break;
                        }
                    }
                }
            }
            class_table
                .inheritance
                .insert(i.name.clone(), inherit_vec.clone());
        }

        // check attribute (optional)

        // check  method
        for i in &self.classes {
            // Main:  Main -> Object -> A

            if let Some(v) = class_table.inheritance.get(&(i.name.clone())) {
                for curr_parent in v.iter().rev() {
                    if DEBUG {
                        println!(" -> {}", &curr_parent.name);
                    }

                    for feature in &curr_parent.features {
                        match feature {
                            Feature::Method(method_) => {
                                if i.features.contains(&feature) {
                                    // check returan_type and attr
                                    let index =
                                        i.features.iter().position(|r| r == feature).unwrap();
                                    if !i.features[index].check_param(&feature)
                                        || !i.features[index].check_return_type(&feature)
                                    {
                                        return Err(SemanticError {
                                            err_msg:format!("{}:{}:{} ---> An error occurred in the parameter type or return type of the method <{}> overridden by Class {}!",
                                                i.file_name,i.features[index].get_position().0,i.features[index].get_position().1,method_.name,i.name),
                                        });
                                    }
                                }
                            }
                            Feature::Attribute(attr) => {
                                if &curr_parent.name != &i.name && i.features.contains(&feature) {
                                    return Err(SemanticError { err_msg: format!("{}:{}:{} ---> You cannot define the same field <{}> in the subclass {} as the superclass {}",
                                        i.file_name,attr.position.0,attr.position.1,attr.name,i.name,curr_parent.name) });
                                }
                            }
                        }
                    }
                }
            }
        }

        // check all expression
        if DEBUG {
            println!();
            println!("Now check all expression");
        }
        // mut to add type to expression;
        for i in &mut self.classes {
            if DEBUG {
                println!("current class is {}", i.name);
            }

            self.symbol_table.enter_scope();
            self.symbol_table.add(&SELF.to_string(), &i.name);
            if let Some(v) = class_table.inheritance.get(&(i.name.clone())) {
                for curr_parent in v.iter() {
                    for feature in &curr_parent.features {
                        if let Feature::Attribute(attr) = feature {
                            self.symbol_table
                                .add(&attr.name, &attr.type_.clone().unwrap())
                        }
                    }
                }
            }

            for j in &mut i.features {
                if let Feature::Method(method) = j {
                    self.symbol_table.enter_scope();
                    for param in &*method.param {
                        self.symbol_table.add(&param.0, &param.1);
                    }
                    if let Some(v) = method.body.deref_mut() {
                        let mut return_ = false;
                        for expr in v {
                            match expr {
                                Expr::Return(re) => {
                                    return_ = true;
                                    match re.check_type(&mut self.symbol_table, class_table) {
                                        Err(e) => return Err(e),
                                        Ok(type_) => {
                                            if !class_table
                                                .is_less_or_equal(&type_, &method.return_type)
                                            {
                                                return Err(SemanticError {
                                                     err_msg: format!("{}:{}:{} ---> The return type of your {} method is different from the declared type!",
                                                                i.file_name,re.position.0,re.position.1,method.name)
                                                    }
                                                );
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    if let Err(e) =
                                        expr.check_type(&mut self.symbol_table, class_table)
                                    {
                                        return Err(SemanticError {
                                            err_msg: format!("{}:{}", i.file_name, e.err_msg),
                                        });
                                    }
                                }
                            }
                        }
                        if !return_ {
                            return Err(SemanticError {
                                err_msg: format!(
                                    "{}:{}:{} ---> Your method needs a return expression, even though you may return in an if or while.",
                                    i.file_name, method.position.0, method.position.1
                                ),
                            });
                        }
                    }
                    self.symbol_table.exit_scope();
                }
            }
            self.symbol_table.exit_scope();
        }

        return Ok(self.classes.clone());
    }
}
