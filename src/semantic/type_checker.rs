use std::ops::{Deref, DerefMut};

use crate::{
    parser::ast::{
        class::{Feature, Ownership},
        expr::{
            Assignment, ComputeOp, Cond, ConstructorCall, Dispatch, DispatchExpr, Expr, For,
            Isnull, Let, Math, MathOp, Not, Return, While,
        },
        is_primitive, Identifier, Type,
    },
    utils::{
        table::{ClassTable, SymbolTable},
        util::do_vecs_match,
    },
    BOOL, INT, OBJECT, SELF, STR, STRING, VOID,
};

use super::semantic::SemanticError;

pub trait TypeChecker {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        unreachable!()
    }
}

impl TypeChecker for Expr {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        match self {
            Expr::Bool(_) => return Ok(BOOL.to_string()),
            Expr::Str(_) => return Ok(STR.to_string()),
            Expr::Int(_) => return Ok(INT.to_string()),
            Expr::New(constructor_call) => {
                return constructor_call.check_type(symbol_table, class_table)
            }

            Expr::ASM(_) => {
                return Ok(OBJECT.to_string());
            }

            Expr::Identifier(e) => {
                if let Some(s) = symbol_table.find(&e.name) {
                    e.type_ = s.clone();
                    return Ok(s.clone());
                } else {
                    return Err(SemanticError::new(
                        format!(
                            "The identifier {} does not exist or it has gone out of scope!",
                            e.name
                        ),
                        Some(e.pos),
                    ));
                }
            }

            Expr::Self_(e) => {
                if let Some(s) = symbol_table.find(&SELF.to_string()) {
                    e.type_ = s.clone();
                    return Ok(s.clone());
                }
                return Ok(OBJECT.to_string());
            }

            Expr::Let(e) => return e.check_type(symbol_table, class_table),

            Expr::Assignment(e) => return e.check_type(symbol_table, class_table),

            Expr::Dispatch(e) => return e.check_type(symbol_table, class_table),

            Expr::Math(e) => return e.check_type(symbol_table, class_table),

            Expr::Cond(e) => return e.check_type(symbol_table, class_table),

            Expr::While(e) => return e.check_type(symbol_table, class_table),

            Expr::Return(e) => return e.check_type(symbol_table, class_table),

            Expr::Not(e) => return e.check_type(symbol_table, class_table),

            Expr::Isnull(e) => return e.check_type(symbol_table, class_table),

            Expr::For(e) => return e.check_type(symbol_table, class_table),

            _ => {}
        }
        return Ok(OBJECT.to_string());
    }
}

impl TypeChecker for ConstructorCall {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        if let Some(_) = class_table.classes.get(&self.class_name) {
            // find the class
            match &mut self.param {
                // check params
                Some(params) => {
                    let mut type_vec: Vec<Type> = vec![];
                    for i in params.deref_mut() {
                        match i.check_type(symbol_table, class_table) {
                            Ok(type_) => {
                                type_vec.push(type_.to_string());
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    let constructor_decls = class_table
                        .class_constructors
                        .get(&self.class_name)
                        .unwrap();
                    for decl in constructor_decls {
                        //  is there a constructor match?
                        let decl_type = decl.param.deref();
                        let iter = decl_type.iter().map(|x| x.1.clone());
                        if do_vecs_match(&type_vec, &(iter.collect())) {
                            return Ok(self.class_name.clone());
                        }
                    }
                    return Err(SemanticError::new(
                        format!(
                            "class {} has no constructor that takes ({:?}) as parameters!",
                            &self.class_name, type_vec
                        ),
                        Some(self.position.clone()),
                    ));
                }
                None => return Ok(self.class_name.clone()),
            }
        } else {
            return Err(SemanticError::new(
                format!("There is no class called {}!", &self.class_name),
                None,
            ));
        }
    }
}

impl TypeChecker for Dispatch {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        let target_type = self.target.check_type(symbol_table, class_table)?;

        if is_primitive(&target_type) {
            return Err(SemanticError::new(
                "Raw type can't use <.> operator!".to_owned(),
                Some(self.position.clone()),
            ));
        }

        if let Some(class_) = class_table.get_classes().get(&target_type) {
            if let Some(v) = class_table.get_inheritance().get(&class_.name) {
                for curr_parent in v {
                    match &mut self.expr {
                        DispatchExpr::Method(method_call) => {
                            for f in &curr_parent.features {
                                if let Feature::Method(method) = f {
                                    if method.name == method_call.fun_name {
                                        let flag = match method.ownership {
                                            Ownership::Private if !self.target.is_self_expr() => {
                                                false
                                            }
                                            _ => true,
                                        };

                                        if flag {
                                            let method_param = method.param.deref();
                                            let actuals = method_call.actual.deref_mut();
                                            if actuals.len() != method_param.len() {
                                                return Err(SemanticError::new(
                                                    "The actual number of parameters is not equal to the number of declared formal parameters!".to_string(),
                                                    Some(self.position.clone()),
                                                ));
                                            }

                                            for (actual, param) in
                                                actuals.iter_mut().zip(method_param.iter())
                                            {
                                                let actual_type =
                                                    actual.check_type(symbol_table, class_table)?;
                                                if !class_table
                                                    .is_less_or_equal(&actual_type, &param.1)
                                                {
                                                    return Err(SemanticError::new(
                                                        "The actual parameter type is not the same as the declared formal parameter type!".to_owned(),
                                                        Some(self.position.clone()),
                                                    ));
                                                }
                                            }

                                            self.type_ = method.return_type.clone();
                                            return Ok(method.return_type.clone());
                                        }
                                    }
                                }
                            }
                        }
                        DispatchExpr::Field(field) => {
                            for f in &curr_parent.features {
                                if let Feature::Attribute(attr) = f {
                                    if attr.name == *field {
                                        let flag = match attr.ownership {
                                            Ownership::Public => true,
                                            Ownership::Private if self.target.is_self_expr() => {
                                                true
                                            }
                                            _ => false,
                                        };

                                        if flag {
                                            self.type_ = attr.type_.clone().unwrap();
                                            return Ok(self.type_.clone());
                                        } else {
                                            return Err(SemanticError::new(
                                                format!("The field {} is private!", field),
                                                Some(self.position.clone()),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return Err(SemanticError::new(
                    format!(
                        "Class {} may not have the method or field you want!",
                        class_.name
                    ),
                    Some(self.position.clone()),
                ));
            }
        }

        Ok(OBJECT.to_string())
    }
}

impl TypeChecker for Let {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        for i in self.var_decls.deref_mut() {
            match i.init.deref_mut() {
                Some(e) => match e.check_type(symbol_table, class_table) {
                    Ok(type_) => {
                        if let Some(decl_type) = &i.type_ {
                            if class_table.is_less_or_equal(&type_, decl_type) {
                                symbol_table.add(&i.name, decl_type);
                            } else {
                                return Err(SemanticError::new(format!("The type of your let expression init is inconsistent with the declared type!",),Some(i.position.clone()) ));
                            }
                        } else {
                            i.type_ = Some(type_.clone());
                            symbol_table.add(&i.name, &type_);
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                },
                None => {
                    if let Some(decl_type) = &i.type_ {
                        symbol_table.add(&i.name, decl_type);
                    }
                }
            }
        }
        return Ok(OBJECT.to_string());
    }
}

impl TypeChecker for Assignment {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        let compute_type = (*self.compute).check_type(symbol_table, class_table);
        if let Some(id_type) = symbol_table.find(&self.id) {
            if let Ok(t) = compute_type {
                if class_table.is_less_or_equal(id_type, &t) {
                    return Ok(id_type.clone());
                }
            }
        }
        return Err(SemanticError::new(
            format!("Some semantic errors occurred in your Assignment!",),
            Some(self.position.clone()),
        ));
    }
}

impl TypeChecker for Math {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        let left_type = (*self.left).check_type(symbol_table, class_table);
        let right_type = (*self.right).check_type(symbol_table, class_table);

        match left_type {
            Ok(left) => match right_type {
                Ok(right) => {
                    if left == INT.to_string() && right == INT.to_string() {
                        match self.op.deref() {
                            MathOp::ComputeOp(_) => {
                                self.type_ = INT.to_string();
                                return Ok(INT.to_string());
                            }
                            MathOp::CondOp(_) => {
                                self.type_ = BOOL.to_string();
                                return Ok(BOOL.to_string());
                            }
                        }
                    }
                    // ! for string plus
                    // else if left == STRING.to_string() && right == STRING.to_string() {
                    //     match self.op.deref() {
                    //         MathOp::ComputeOp(op_) => {
                    //             if let ComputeOp::Add = op_ {
                    //                 self.type_ = STRING.to_string();
                    //                 return Ok(STRING.to_string());
                    //             } else {
                    //                 return Err(SemanticError::new(
                    //                     format!(
                    //                         "String cannot be used for mathematical operations other than addition"
                    //                     ),
                    //                     None,
                    //                 ));
                    //             }
                    //         }
                    //         MathOp::CondOp(_) => {
                    //             return Err(SemanticError::new(
                    //                 format!("String cannot be used in conditional operations"),
                    //                 None,
                    //             ))
                    //         }
                    //     }
                    // }
                    else {
                        return Err(SemanticError::new(
                            format!("The left and right sides of your mathematical operation are not all INT types!"),
                            None
                        ));
                    }
                }
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }
}

impl TypeChecker for Cond {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        symbol_table.enter_scope();

        let test_type = (*self.test).check_type(symbol_table, class_table);
        match test_type {
            Ok(test) => {
                if test != BOOL.to_string() {
                    return Err(SemanticError::new(
                        format!("The type in your If condition is not BOOL",),
                        Some(self.position.clone()),
                    ));
                }
            }
            Err(e) => return Err(e),
        }
        for then_expr in self.then_body.deref_mut() {
            let then_type = then_expr.check_type(symbol_table, class_table);
            match then_type {
                Err(e) => return Err(e),
                _ => {}
            }
        }

        for else_expr in self.else_body.deref_mut() {
            let else_type = else_expr.check_type(symbol_table, class_table);
            match else_type {
                Err(e) => return Err(e),
                _ => {}
            }
        }

        symbol_table.exit_scope();
        return Ok(OBJECT.to_string());
    }
}

impl TypeChecker for While {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        symbol_table.enter_scope();

        let test_type = (*self.test).check_type(symbol_table, class_table);
        match test_type {
            Ok(test) => {
                if test != BOOL.to_string() {
                    return Err(SemanticError::new(
                        format!("The type in your Loop condition is not BOOL",),
                        Some(self.position.clone()),
                    ));
                }
            }
            Err(e) => return Err(e),
        }
        for body_expr in self.body.deref_mut() {
            let body_type = body_expr.check_type(symbol_table, class_table);
            match body_type {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        symbol_table.exit_scope();
        return Ok(OBJECT.to_string());
    }
}

impl TypeChecker for Return {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        match &mut self.val {
            Some(e) => match e.deref_mut().check_type(symbol_table, class_table) {
                Ok(e) => return Ok(e),
                Err(e) => return Err(e),
            },
            None => return Ok(VOID.to_string()),
        }
    }
}

impl TypeChecker for Not {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        let e = self.expr.deref_mut();
        let expr_type = e.check_type(symbol_table, class_table);
        match expr_type {
            Ok(type_) => {
                if type_ != BOOL.to_string() {
                    return Err(SemanticError::new(
                        format!("The type in your Not expression is not BOOL",),
                        Some(self.position.clone()),
                    ));
                }
                return Ok(type_);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

impl TypeChecker for Isnull {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        let e = self.expr.deref_mut();
        let expr_type = e.check_type(symbol_table, class_table);
        match expr_type {
            Ok(_) => {
                return Ok(BOOL.to_string());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

impl TypeChecker for For {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        symbol_table.enter_scope();

        if self.init.deref().len() > 1 || self.test.deref().len() > 1 {
            return Err(SemanticError::new(
                format!(
                    " There can only be one initial expression and one judgment expression in the for loop!"
                ),
                Some(self.position)
            ));
        }
        for init_ in self.init.deref_mut() {
            let init_type = init_.check_type(symbol_table, class_table);
            match init_type {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        for test_ in self.test.deref_mut() {
            let test_type: Result<String, SemanticError> =
                test_.check_type(symbol_table, class_table);
            match test_type {
                Err(e) => return Err(e),
                Ok(type_) => {
                    if type_ != BOOL.to_string() {
                        return Err(SemanticError::new(
                            format!(
                                "The type of the conditional expression in the for loop is not BOOL!"
                            ),
                            Some(self.position)
                        ));
                    }
                }
            }
        }
        for iter_ in self.iter.deref_mut() {
            let iter_type = iter_.check_type(symbol_table, class_table);
            match iter_type {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        for body_ in self.body.deref_mut() {
            let body_type = body_.check_type(symbol_table, class_table);
            match body_type {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        symbol_table.exit_scope();
        Ok(OBJECT.to_string())
    }
}
