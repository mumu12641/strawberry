use crate::{ast::Class, table::ClassTable};

pub struct SemanticChecker {
    classes: Vec<Class>,
}

pub struct SemanticError {
    pub err_msg: String,
}

impl SemanticChecker {
    // * install constants and basic classes.
    // * get all classes not just user defined but also include IO, Object and so on.
    // * check Main
    // * check inherit
    // * check attributes
    // * check method override
    // * check all expressions

    pub fn new(classes_: Vec<Class>) -> SemanticChecker {
        SemanticChecker { classes: classes_ }
    }
    pub fn check(&self, class_table: &mut ClassTable) -> Result<bool, SemanticError> {
        let mut main_flag = false;

        // check repeat class
        for i in &self.classes {
            if i.name == "Main".to_string() {
                main_flag = true;
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

        // check inheritance
        // class_table.inheritance
        for i in &self.classes {
            let mut inherit_vec: Vec<Class> = Vec::new();
            let mut curr_parent: Option<String>;
            curr_parent = i.parent.clone();
            loop {
                match curr_parent {
                    Some(s) => {
                        let str = s.clone();
                        if s.clone() == "None".to_string() {
                            // current is object
                            break;
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
            // while
        }

        // Ok()
        return Ok(true);
    }
}
