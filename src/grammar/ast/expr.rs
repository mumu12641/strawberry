use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    grammar::lexer::Position,
    semantic::semantic::SemanticError,
    utils::table::{ClassTable, SymbolTable},
    BOOL, INT, OBJECT, SELF, STRING,
};

use super::{
    class::{Feature, VarDecl},
    Boolean, Identifier, Int, Str, Type,
};

#[derive(Debug, Clone)]
pub enum MathOp {
    ComputeOp(ComputeOp),
    CondOp(CondOp),
}
#[derive(Debug, Clone)]
pub enum ComputeOp {
    Add,
    Minus,
    Mul,
    Divide,
}
#[derive(Debug, Clone)]
pub enum CondOp {
    Equal,
    More,
    MoreE,
    Less,
    LessE,
}

#[derive(Debug, Clone)]
pub struct Dispatch {
    pub target: Box<Option<Expr>>,
    pub fun_name: Identifier,
    pub actual: Box<Vec<Expr>>,
    pub position: Position,
    pub type_: Type,
}
#[derive(Debug, Clone)]
pub struct Cond {
    pub test: Box<Expr>,
    pub then_body: Box<Vec<Expr>>,
    pub else_body: Box<Vec<Expr>>,
    pub position: Position,
}
#[derive(Debug, Clone)]
pub struct While {
    pub test: Box<Expr>,
    pub body: Box<Vec<Expr>>,
    pub position: Position,
}
#[derive(Debug, Clone)]
pub struct Math {
    pub left: Box<Expr>,
    pub op: Box<MathOp>,
    pub right: Box<Expr>,
    pub type_: Type,
}
#[derive(Debug, Clone)]
pub struct Return {
    pub val: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub var_decls: Box<Vec<VarDecl>>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: Identifier,
    pub compute: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub struct IdentifierSrtuct {
    pub name: Identifier,
    pub pos: Position,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct Not {
    pub expr: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub struct Isnull {
    pub expr: Box<Expr>,
    // pub position: Position,
}

#[derive(Debug, Clone)]
pub struct Self_ {
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct For {
    pub init: Box<Vec<Expr>>,
    pub test: Box<Vec<Expr>>,
    pub iter: Box<Vec<Expr>>,
    pub body: Box<Vec<Expr>>,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(IdentifierSrtuct),
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Assignment),

    Dispatch(Dispatch),
    Cond(Cond),
    While(While),
    For(For),
    Block(Box<Vec<Expr>>),
    Let(Let),
    New(Type),
    Self_(Self_),
    Isvoid(Box<Expr>),

    Math(Math),
    Not(Not),
    Isnull(Isnull),

    Return(Return),
}
pub trait TypeChecker: Debug {
    fn check_type(
        &mut self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError>;
}

pub trait TypeGet: Debug {
    fn get_type(&self) -> Type;
}

impl TypeGet for Expr {
    fn get_type(&self) -> Type {
        match self {
            Expr::Bool(_) => return BOOL.to_string(),
            Expr::Str(_) => return STRING.to_string(),
            Expr::Int(_) => return INT.to_string(),
            Expr::New(type_) => return type_.clone(),
            Expr::Identifier(e) => return e.type_.clone(),
            Expr::Dispatch(e) => return e.type_.clone(),
            Expr::Self_(e) => return e.type_.clone(),
            Expr::Math(e) => return e.type_.clone(),
            _ => return OBJECT.to_string(),
        }
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
            Expr::Str(_) => return Ok(STRING.to_string()),
            Expr::Int(_) => return Ok(INT.to_string()),
            Expr::New(type_) => return Ok(type_.clone()),

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
        match self.target.deref_mut() {
            Some(e) => {
                match e.check_type(symbol_table, class_table) {
                    Ok(target_type) => {
                        if let Some(class) = class_table.get_classes().get(&target_type) {
                            if let Some(v) = class_table.get_inheritance().get(&class.name) {
                                let mut find = false;
                                for class in v {
                                    for f in &class.features {
                                        if let Feature::Method(method) = f {
                                            if &method.name == &self.fun_name {
                                                find = true;
                                                let method_param = method.param.deref();
                                                let actuals = self.actual.deref_mut();
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
                                if !find {
                                    return Err(SemanticError {
                                        err_msg: format!(
                                            "{}:{} ---> The expression no method called {}() !",
                                            self.position.0, self.position.1, &self.fun_name
                                        ),
                                    });
                                }
                            }
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            None => todo!(),
        }
        // class_table.classes.get(self.target)
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
