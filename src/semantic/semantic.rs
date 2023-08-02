use std::ops::DerefMut;

use crate::{
    // grammar::ast::{Class, Feature, MethodDecl},
    grammar::{
        ast::{
            class::{Class, ConstructorDecl, Feature},
            expr::{Expr, TypeChecker},
            Identifier, Type,
        },
        lexer::Position,
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
    pub position: Option<Position>,
    pub file_name: String,
}

impl SemanticError {
    pub fn new(err_msg_: String, position_: Option<Position>) -> SemanticError {
        SemanticError {
            err_msg: err_msg_,
            position: position_,
            file_name: "".to_string(),
        }
    }
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
        //* check repeat class */
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
                    err_msg: format!("Class {} has been redefined!", i.name),
                    file_name: i.file_name.clone(),
                    position: Some(i.position),
                });
            } else {
                class_table.classes.insert(i.name.clone(), i.clone());
            }
        }

        //* chech main */
        if !main_flag {
            return Err(SemanticError {
                err_msg: format!("Your program is missing the Main class"),
                file_name: "".to_string(),
                position: None,
            });
        }
        if !main_method_flag {
            return Err(SemanticError {
                err_msg: format!("Your program is missing the Main function"),
                file_name: "".to_string(),
                position: None,
            });
        }

        //* check inheritance */
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
                                    "There is an inheritance cycle about Class {}!",
                                    s
                                ),
                                file_name: i.file_name.clone(),
                                position: Some(i.position),
                            });
                        } else {
                            if let Some(c) = class_table.classes.get(&(s.clone())) {
                                inherit_vec.insert(0, c.clone());
                                curr_parent = c.parent.clone();
                            } else {
                                return Err(SemanticError {
                                    err_msg: format!(
                                        "Your Class {} inherits an undefined Class {} !",
                                        i.name, s
                                    ),
                                    file_name: i.file_name.clone(),
                                    position: Some(i.position),
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
            class_table.inheritance.insert(i.name.clone(), inherit_vec);
        }

        //* check construtor */
        for i in &self.classes {
            // ! do not clone
            let mut construtor_vec: Vec<ConstructorDecl> = vec![];
            for feature in &i.features {
                match feature {
                    Feature::Constructor(constructor_decl) => {
                        if !construtor_vec.contains(constructor_decl) {
                            construtor_vec.push(constructor_decl.clone());
                        } else {
                            return Err(SemanticError {
                                 err_msg:format!("The parameter declaration for this constructor method duplicates"),
                                  position: Some(constructor_decl.position),
                                  file_name:i.file_name.clone(),
                                 });
                        }
                    }
                    _ => {}
                }
            }
            class_table
                .class_constructors
                .insert(i.name.clone(), construtor_vec);
        }

        //* check  method */
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
                                // just check name, find it if the class override curr_parent's method
                                if i.features.contains(&feature) {
                                    // check returan_type and attr and ownership
                                    let index =
                                        i.features.iter().position(|r| r == feature).unwrap();

                                    // check param
                                    if !i.features[index].check_param(&feature) {
                                        return Err(SemanticError {
                                            err_msg: format!(
                                                "An error occurred in the parameter type of the method <{}> overridden by Class {}!",method_.name,i.name
                                            ),
                                            position: Some(i.features[index].get_position()),
                                            file_name:  i.file_name.clone().clone(),
                                        });
                                    }
                                    // check return type
                                    if !i.features[index].check_return_type(&feature) {
                                        return Err(SemanticError {
                                            err_msg: format!(
                                                "An error occurred in the return type of the method <{}> overridden by Class {}!",method_.name,i.name
                                            ),
                                            position: Some(i.features[index].get_position()),
                                            file_name:  i.file_name.clone(),
                                        });
                                    }
                                    // check ownership
                                    if feature.get_ownership() != i.features[index].get_ownership()
                                    {
                                        return Err(SemanticError {
                                            err_msg: format!(
                                                "An error occurred in the ownership of the method <{}> overridden by Class {}!",method_.name,i.name
                                            ),
                                            position: Some(i.features[index].get_position()),
                                            file_name:  i.file_name.clone(),
                                        });
                                    }
                                }
                            }

                            Feature::Attribute(attr) => {
                                if &curr_parent.name != &i.name && i.features.contains(&feature) {
                                    return Err(SemanticError { err_msg: format!("You cannot define the same field <{}> in the subclass {} as the superclass {}",
                                        attr.name,i.name,curr_parent.name),
                                        file_name:  i.file_name.clone(),
                                        position: Some(i.position),
                                    });
                                }
                            }

                            Feature::Constructor(_) => {}
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
                match j {
                    Feature::Method(method) => {
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
                                            Err(e) => {
                                                return Err(SemanticError {
                                                    err_msg: e.err_msg,
                                                    file_name: i.file_name.clone(),
                                                    position: e.position,
                                                });
                                            }
                                            Ok(type_) => {
                                                if !class_table
                                                    .is_less_or_equal(&type_, &method.return_type)
                                                {
                                                    return Err(SemanticError {
                                                     err_msg: format!("The return type of your {} method is different from the declared type!",
                                                               method.name),
                                                    file_name:  i.file_name.clone(),
                                                    position: Some(re.position),
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
                                                err_msg: e.err_msg,
                                                file_name: i.file_name.clone(),
                                                position: e.position,
                                            });
                                        }
                                    }
                                }
                            }
                            if !return_ {
                                return Err(SemanticError {
                                err_msg: format!(
                                    "Your method needs a return expression, even though you may return in an if or while.",
                                ),
                                file_name:  i.file_name.clone(),
                                position: Some(method.position),
                            });
                            }
                        }
                        self.symbol_table.exit_scope();
                    }
                    Feature::Constructor(constructor) => {
                        self.symbol_table.enter_scope();
                        for param in &*constructor.param {
                            self.symbol_table.add(&param.0, &param.1);
                        }
                        if let Some(v) = constructor.body.deref_mut() {
                            for expr in v {
                                if let Expr::Return(re) = expr {
                                    return Err(SemanticError {
                                        err_msg: format!(
                                            "You cannot add a Return expression in constructor.",
                                        ),
                                        file_name: i.file_name.clone(),
                                        position: Some(re.position),
                                    });
                                }
                                if let Err(e) = expr.check_type(&mut self.symbol_table, class_table)
                                {
                                    return Err(SemanticError {
                                        err_msg: e.err_msg,
                                        file_name: i.file_name.clone(),
                                        position: e.position,
                                    });
                                }
                            }
                        }
                        self.symbol_table.exit_scope();
                    }
                    _ => {}
                }
            }
            self.symbol_table.exit_scope();
        }

        return Ok(self.classes.clone());
    }
}
