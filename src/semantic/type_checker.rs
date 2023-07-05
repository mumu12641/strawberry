use std::ops::{Deref, DerefMut};

use crate::{
    grammar::ast::{
        class::{Feature, Ownership},
        expr::{
            Assignment, ComputeOp, Cond, Dispatch, DispatchExpr, Expr, For, Isnull, Let, Math,
            MathOp, Not, Return, TypeChecker, While,
        },
        Identifier, Type,
    },
    utils::table::{ClassTable, SymbolTable},
    BOOL, INT, OBJECT, SELF, STRING,
};

use super::semantic::SemanticError;

impl TypeChecker for Expr {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        match self {
            Expr::Bool(_) => return Ok(BOOL.to_string()),
            Expr::Str(_) => return Ok(STRING.to_string()),
            Expr::Int(_) => return Ok(INT.to_string()),
            Expr::New(type_) => {
                if let Some(_) = class_table.classes.get(type_) {
                    return Ok(type_.clone());
                } else {
                    return Err(SemanticError {
                        err_msg: format!(
                            "There is no class called {}, maybe you should import it!",
                            type_
                        ),
                    });
                }
            }

            Expr::Identifier(e) => {
                if let Some(s) = symbol_table.find(&e.name) {
                    e.type_ = s.clone();
                    return Ok(s.clone());
                } else {
                    return Err(SemanticError {
                        err_msg: format!("{}:{} ---> The identifier {} does not exist or it has gone out of scope!",e.pos.0,e.pos.1,e.name)
                    });
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

impl TypeChecker for Dispatch {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        match self
            .target
            .deref_mut()
            .check_type(symbol_table, class_table)
        {
            Ok(target_type) => {
                if let Some(class) = class_table.get_classes().get(&target_type) {
                    if let Some(v) = class_table.get_inheritance().get(&class.name) {
                        for class in v {
                            match &mut self.expr {
                                // check method
                                DispatchExpr::Method(method_call) => {
                                    for f in &class.features {
                                        if let Feature::Method(method) = f {
                                            if &method.name == &method_call.fun_name {
                                                let flag ;
                                                if let Ownership::Private = method.ownership {
                                                    if !self.target.deref().is_self_expr() {
                                                        return Err(SemanticError {
                                                            err_msg: format!("{}:{} ---> The method {} in class {} is private!",self.position.0,self.position.1,method.name,class.name),
                                                        });
                                                    } else {
                                                        flag = true;
                                                    }
                                                } else {
                                                    flag = true;
                                                }

                                                if flag {
                                                    let method_param = method.param.deref();
                                                    let actuals = method_call.actual.deref_mut();
                                                    if actuals.len() != method_param.len() {
                                                        return Err(SemanticError { err_msg: format!("{}:{} ---> The actual number of parameters of your method call is not equal to the number of declared formal parameters!",self.position.0,self.position.1), });
                                                    }
                                                    for index in 0..method_param.len() {
                                                        let actual_type = actuals[index]
                                                            .check_type(symbol_table, class_table);
                                                        match actual_type {
                                                            Ok(type_) => {
                                                                if !class_table.is_less_or_equal(
                                                                    &type_,
                                                                    &method_param[index].1,
                                                                ) {
                                                                    return Err(SemanticError { err_msg: format!("{}:{} ---> The actual parameter type of your method call is not the same as the declared formal parameter type!",self.position.0,self.position.1), });
                                                                }
                                                            }
                                                            Err(e) => return Err(e),
                                                        }
                                                    }
                                                    self.type_ = method.return_type.clone();
                                                    return Ok(method.return_type.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                                // check field
                                DispatchExpr::Field(field) => {
                                    for f in &class.features {
                                        if let Feature::Attribute(attr) = f {
                                            if &attr.name == field {
                                                if let Ownership::Public = attr.ownership {
                                                    self.type_ = attr.type_.clone().unwrap();
                                                    return Ok(self.type_.clone());
                                                } else {
                                                    // if target is self, then nobody cares
                                                    // if self.target.deref()
                                                    if self.target.deref().is_self_expr() {
                                                        self.type_ = attr.type_.clone().unwrap();
                                                        return Ok(self.type_.clone());
                                                    } else {
                                                        return Err(SemanticError {
                                                            err_msg: format!("{}:{} ---> The field {} in class {} is private!",self.position.0,self.position.1,field,class.name),
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return Err(SemanticError {
                            err_msg: format!(
                                "{}:{} ---> Some dispatch errors appear !",
                                self.position.0, self.position.1
                            ),
                        });
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
        return Ok(OBJECT.to_string());
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
                                return Err(SemanticError { err_msg: format!("{}:{} ---> The type of your let expression init is inconsistent with the declared type!",i.position.0,i.position.1), });
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
        // type_ <= id.type
        if let Some(id_type) = symbol_table.find(&self.id) {
            // s< compute_type
            if let Ok(t) = compute_type {
                if class_table.is_less_or_equal(id_type, &t) {
                    return Ok(id_type.clone());
                }
            }
        }
        return Err(SemanticError {
            err_msg: format!(
                "{}:{} ---> Some semantic errors occurred in your Assignment!",
                self.position.0, self.position.1
            ),
        });
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
                    } else if left == STRING.to_string() && right == STRING.to_string() {
                        match self.op.deref() {
                            MathOp::ComputeOp(op_) => {
                                if let ComputeOp::Add = op_ {
                                    // self.type_ = STRING.to_string();
                                    self.type_ = STRING.to_string();
                                    return Ok(STRING.to_string());
                                } else {
                                    return Err(SemanticError {
                                        err_msg: format!(
                                            "String cannot be used for mathematical operations other than addition"
                                        ),
                                    });
                                }
                            }
                            MathOp::CondOp(_) => {
                                return Err(SemanticError {
                                    err_msg: format!(
                                        "String cannot be used in conditional operations"
                                    ),
                                })
                            }
                        }
                    } else {
                        return Err(SemanticError {
                        err_msg:
                            format!("The left and right sides of your mathematical operation are not all INT types!"),
                    });
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
                    return Err(SemanticError {
                        err_msg: format!(
                            "{}:{} ---> The type in your If condition is not BOOL",
                            self.position.0, self.position.1
                        ),
                    });
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
                    return Err(SemanticError {
                        err_msg: format!(
                            "{}:{} ---> The type in your Loop condition is not BOOL",
                            self.position.0, self.position.1
                        ),
                    });
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
        match (*self.val).check_type(symbol_table, class_table) {
            Ok(e) => {
                return Ok(e);
            }
            Err(e) => return Err(e),
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
                    return Err(SemanticError {
                        err_msg: format!(
                            "{}:{} ---> The type in your Not expression is not BOOL",
                            self.position.0, self.position.1,
                        ),
                    });
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
            return Err(SemanticError {
                err_msg: format!(
                    "{}:{} ---> There can only be one initial expression and one judgment expression in the for loop!",self.position.0,self.position.1
                ),
            });
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
                        return Err(SemanticError {
                            err_msg: format!(
                                "{}:{} ---> The type of the conditional expression in the for loop is not BOOL!",self.position.0,self.position.1
                            ),
                        });
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
