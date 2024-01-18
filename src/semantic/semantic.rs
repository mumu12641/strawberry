use std::{cell::RefCell, collections::HashSet, ops::DerefMut};

use crate::{
    ctx::CompileContext,
    lexer::Position,
    // parser::ast::{Class, Feature, MethodDecl},
    parser::ast::{
        class::{Class, ConstructorDecl, Feature},
        expr::Expr,
        Identifier, Type,
    },
    table::ClassTable,
    utils::table::SymbolTable,
    DEBUG,
    SELF,
};

use super::type_checker::TypeChecker;

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

/// * install constants and basic classes.
/// * get all classes not just user defined but also include IO, Object and so on.
/// * check Main
/// * check inherit
/// * check attributes
/// * check method override
/// * check all expressions
pub struct SemanticChecker {
    symbol_table: SymbolTable<Identifier, Type>,
    pub ctx: CompileContext,
}
impl SemanticChecker {
    pub fn new(ctx: CompileContext) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            ctx,
        }
    }
    pub fn check(&mut self) -> Result<Vec<Class>, SemanticError> {
        let mut main_flag = false;
        let mut main_method_flag = false;

        //* check repeat class */
        for i in &self.ctx.classes {
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
            if self.ctx.class_table.classes.contains_key(&i.name) {
                return Err(SemanticError {
                    err_msg: format!("Class {} has been redefined!", i.name),
                    file_name: i.file_name.clone(),
                    position: Some(i.position),
                });
            } else {
                self.ctx
                    .class_table
                    .classes
                    .insert(i.name.clone(), i.clone());
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
        for i in &self.ctx.classes {
            let mut inherit_vec: Vec<Class> = Vec::new();
            inherit_vec.insert(0, i.clone());
            let mut curr_parent = i.parent.clone();

            while let Some(ref parent_name) = curr_parent {
                if parent_name == "None" {
                    // current is object
                    break;
                } else if parent_name == &i.name {
                    return Err(SemanticError {
                        err_msg: format!(
                            "There is an inheritance cycle about Class {}!",
                            parent_name
                        ),
                        file_name: i.file_name.clone(),
                        position: Some(i.position),
                    });
                } else {
                    let parent_class = self.ctx.class_table.classes.get(parent_name);
                    match parent_class {
                        Some(parent_class) => {
                            inherit_vec.insert(0, parent_class.clone());
                            curr_parent = parent_class.parent.clone();
                        }
                        None => {
                            return Err(SemanticError {
                                err_msg: format!(
                                    "Your Class {} inherits an undefined Class {} !",
                                    i.name, parent_name
                                ),
                                file_name: i.file_name.clone(),
                                position: Some(i.position),
                            });
                        }
                    }
                }
            }

            if let Some(object_class) = self.ctx.class_table.classes.get(&"Object".to_string()) {
                inherit_vec.insert(0, object_class.clone());
            }

            self.ctx
                .class_table
                .inheritance
                .insert(i.name.clone(), inherit_vec);
        }

        //* check construtor */
        for i in &self.ctx.classes {
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
            self.ctx
                .class_table
                .class_constructors
                .insert(i.name.clone(), construtor_vec);
        }

        //* check  method */
        for i in &self.ctx.classes {
            // Main:  Main -> Object -> A

            if let Some(v) = self.ctx.class_table.inheritance.get(&(i.name.clone())) {
                for curr_parent in v.iter().rev() {
                    for feature in &curr_parent.features {
                        match feature {
                            Feature::Method(method_) => {
                                //*  just check name, find it if the class override curr_parent's method
                                if i.features.contains(&feature) {
                                    //* check returan_type and attr and ownership
                                    let index =
                                        i.features.iter().position(|r| r == feature).unwrap();

                                    //* check param
                                    if !i.features[index].check_param(&feature) {
                                        return Err(SemanticError {
                                            err_msg: format!(
                                                "An error occurred in the parameter type of the method <{}> overridden by Class {}!",method_.name,i.name
                                            ),
                                            position: Some(i.features[index].get_position()),
                                            file_name:  i.file_name.clone().clone(),
                                        });
                                    }
                                    //* check return type
                                    if !i.features[index].check_return_type(&feature) {
                                        return Err(SemanticError {
                                            err_msg: format!(
                                                "An error occurred in the return type of the method <{}> overridden by Class {}!",method_.name,i.name
                                            ),
                                            position: Some(i.features[index].get_position()),
                                            file_name:  i.file_name.clone(),
                                        });
                                    }
                                    //* check ownership
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

        //* mut to add type to expression;
        for i in &mut self.ctx.classes {
            self.symbol_table.enter_scope();
            self.symbol_table.add(&SELF.to_string(), &i.name);

            if let Some(v) = self.ctx.class_table.inheritance.get(&(i.name.clone())) {
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
                                        match re.check_type(
                                            &mut self.symbol_table,
                                            &mut self.ctx.class_table,
                                        ) {
                                            Err(e) => {
                                                return Err(SemanticError {
                                                    err_msg: e.err_msg,
                                                    file_name: i.file_name.clone(),
                                                    position: e.position,
                                                });
                                            }
                                            Ok(type_) => {
                                                if !self
                                                    .ctx
                                                    .class_table
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
                                        if let Err(e) = expr.check_type(
                                            &mut self.symbol_table,
                                            &mut self.ctx.class_table,
                                        ) {
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
                                if let Err(e) = expr
                                    .check_type(&mut self.symbol_table, &mut self.ctx.class_table)
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
                    //*  check attribute type
                    Feature::Attribute(attr) => {
                        self.symbol_table.enter_scope();
                        if let Some(init_expr) = attr.init.deref_mut() {
                            match init_expr
                                .check_type(&mut self.symbol_table, &mut self.ctx.class_table)
                            {
                                Ok(init_type) => {
                                    if let Some(attr_type) = &attr.type_ {
                                        if !self
                                            .ctx
                                            .class_table
                                            .is_less_or_equal(attr_type, &init_type)
                                        {
                                            return Err(SemanticError {
                                                err_msg: format!(
                                                "Some semantic errors occurred in your Assignment!"
                                            ),
                                                position: Some(attr.position),
                                                file_name: i.file_name.clone(),
                                            });
                                        }
                                    }
                                }
                                Err(e) => {
                                    return Err(SemanticError {
                                        err_msg: e.err_msg,
                                        file_name: i.file_name.clone(),
                                        position: e.position,
                                    })
                                }
                            }
                        }
                        self.symbol_table.exit_scope();
                    }
                }
            }
            self.symbol_table.exit_scope();
        }

        return Ok(self.ctx.classes.clone());
    }
}
