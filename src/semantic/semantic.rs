use crate::{
    // grammar::ast::{Class, Feature, MethodDecl},
    grammar::ast::{
        class::{Class, Feature},
        expr::TypeChecker,
        Identifier, Type,
    },
    table::ClassTable,
    utils::table::SymbolTable,
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
    pub fn check(&mut self, class_table: &mut ClassTable) -> Result<bool, SemanticError> {
        let mut main_flag = false;
        let mut main_method_flag = false;

        // check repeat class
        for i in &self.classes {
            if i.name == "Main".to_string() {
                main_flag = true;
                for feature in &i.features {
                    if let Feature::Method(m) = feature {
                        if m.name.clone() == "main".to_string() {
                            main_method_flag = true;
                        }
                    }
                }
            }
            if class_table.classes.contains_key(&i.name) {
                return Err(SemanticError {
                    err_msg: "Class ".to_string() + &i.name.clone() + " has been redefined!",
                });
            } else {
                class_table.classes.insert(i.name.clone(), i.clone());
            }
        }

        // chech main
        if !main_flag {
            return Err(SemanticError {
                err_msg: "Your program is missing the Main class".to_string(),
            });
        }
        if !main_method_flag {
            return Err(SemanticError {
                err_msg: "Your program is missing the Main function".to_string(),
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
                        } else if s.clone() == i.name.clone() {
                            return Err(SemanticError {
                                err_msg: "There is an inheritance cycle about class ".to_string()
                                    + &s.clone()
                                    + " !",
                            });
                        } else {
                            if let Some(c) = class_table.classes.get(&(s.clone())) {
                                inherit_vec.insert(0, c.clone());
                                curr_parent = c.parent.clone();
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
        println!();
        for i in &self.classes {
            // Main: Object -> A

            println!("{} inheritance diagram", &i.name);
            if let Some(v) = class_table.inheritance.get(&(i.name.clone())) {
                for curr_parent in v.iter().rev() {
                    println!(" -> {}", &curr_parent.name);

                    for feature in &curr_parent.features {
                        match feature {
                            Feature::Method(method_) => {
                                if i.features.contains(&feature) {
                                    // check returan_type and attr
                                    let index =
                                        i.features.iter().position(|r| r == feature).unwrap();
                                    if !i.features[index].clone().check_param(&feature)
                                        || !i.features[index].clone().check_return_type(&feature)
                                    {
                                        return Err(SemanticError {
                                            err_msg: "An error occurred in the parameter type or return type of the method <"
                                                .to_string()
                                                + &method_.name
                                                + "> overridden by class " + &i.name + "!",
                                        });
                                    }
                                }
                            }
                            // TODO: check attr
                            _ => {}
                        }
                    }
                }
            }
        }

        // check all expression
        println!();
        println!("Now check all expression");
        for i in &self.classes {
            println!("current class is {}", i.name);

            self.symbol_table.enter_scope();
            if let Some(v) = class_table.inheritance.get(&(i.name.clone())) {
                for curr_parent in v.iter().rev() {
                    for feature in &curr_parent.features {
                        if let Feature::Attribute(attr) = feature {
                            self.symbol_table.add(&attr.name, &attr.type_)
                        }
                    }
                }
            }
            for j in &i.features {
                if let Feature::Method(method) = j {
                    for param in *method.param.clone() {
                        self.symbol_table.add(&param.0, &param.1);
                    }
                    if let Some(v) = *method.body.clone() {
                        for expr in v {
                            expr.check_type(&mut self.symbol_table);
                        }
                    }
                }
            }
            self.symbol_table.debug();
            self.symbol_table.exit_scope();
        }

        return Ok(true);
    }
}
