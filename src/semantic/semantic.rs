use crate::{
    grammar::ast::{Class, Feature},
    table::ClassTable,
};

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
        for i in &self.classes {
            // Main: Object -> A
            println!("{}", &i.name);
            if let Some(v) = class_table.inheritance.get(&(i.name.clone())) {
                for curr_parent in v.iter().rev() {
                    println!(" -> {}", &curr_parent.name);

                    for feature in &curr_parent.features {
                        match feature {
                            Feature::Method(method_) => {
                                if i.features.contains(&feature) {
                                    // check returan_type and attr
                                    // if i.features.

                                    // TODO: if object has a fuction , these will be an error
                                } else {
                                    return Err(SemanticError {
                                        err_msg: "An error occurred in the parameter type or return type of the method <"
                                            .to_string()
                                            + &method_.name
                                            + "> overridden by class " + &i.name + "!",
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        return Ok(true);
    }
}
