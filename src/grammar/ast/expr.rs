use std::fmt::Debug;

use crate::{
    grammar::lexer::Position,
    semantic::semantic::SemanticError,
    utils::table::{ClassTable, SymbolTable},
    BOOL, INT, OBJECT, STRING,
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
}
#[derive(Debug, Clone)]
pub struct Cond {
    pub test: Box<Expr>,
    pub then_body: Box<Vec<Expr>>,
    pub else_body: Box<Vec<Expr>>,
    pub postion: Position,
}
#[derive(Debug, Clone)]
pub struct While {
    pub test: Box<Expr>,
    pub body: Box<Vec<Expr>>,
    pub postion: Position,
}
#[derive(Debug, Clone)]
pub struct Math {
    pub left: Box<Expr>,
    pub op: Box<MathOp>,
    pub right: Box<Expr>,
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
pub enum Expr {
    Identifier(Identifier, Position),
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Assignment),

    Dispatch(Dispatch),
    Cond(Cond),
    While(While),
    Block(Box<Vec<Expr>>),
    Let(Let),
    New(Type),
    Self_(Identifier),
    Isvoid(Box<Expr>),

    Math(Math),

    Return(Return),
}
pub trait TypeChecker: Debug {
    fn check_type(
        &self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError>;
}
impl TypeChecker for Expr {
    fn check_type(
        &self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        match self {
            Expr::Bool(_) => return Ok(BOOL.to_string()),
            Expr::Str(_) => return Ok(STRING.to_string()),
            Expr::Int(_) => return Ok(INT.to_string()),
            Expr::New(type_) => return Ok(type_.clone()),

            Expr::Identifier(e, pos) => {
                if let Some(s) = symbol_table.find(e) {
                    return Ok(s.clone());
                } else {
                    return Err(SemanticError {
                        err_msg: format!("{}:{} ---> The identifier {} does not exist or it has gone out of scope!",pos.0,pos.1,e)
                    });
                }
            }
            Expr::Let(e) => return e.check_type(symbol_table, class_table),

            Expr::Assignment(e) => return e.check_type(symbol_table, class_table),

            Expr::Dispatch(e) => return e.check_type(symbol_table, class_table),

            Expr::Math(e) => return e.check_type(symbol_table, class_table),

            Expr::Cond(e) => return e.check_type(symbol_table, class_table),

            Expr::While(e) => return e.check_type(symbol_table, class_table),

            Expr::Return(e) => return e.check_type(symbol_table, class_table),

            _ => {}
        }
        return Ok(OBJECT.to_string());
    }
}

impl TypeChecker for Dispatch {
    fn check_type(
        &self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        match *(self.target.clone()) {
            Some(e) => {
                if let Ok(target_type) = e.check_type(symbol_table, class_table) {
                    if let Some(class) = class_table.get_classes().get(&target_type) {
                        if let Some(v) = class_table.get_inheritance().get(&class.name) {
                            for class in v {
                                for f in &class.features {
                                    if let Feature::Method(method) = f {
                                        if &method.name == &self.fun_name {
                                            let method_param = *(method.param.clone());
                                            let actuals = *(self.actual.clone());
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
                                            return Ok(method.return_type.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        // class_table.classes.get(self.target)
        return Ok(OBJECT.to_string());
    }
}

impl TypeChecker for Let {
    fn check_type(
        &self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        for i in *(self.var_decls.clone()) {
            match *(i.init.clone()) {
                Some(e) => match e.check_type(symbol_table, class_table) {
                    Ok(type_) => {
                        if class_table.is_less_or_equal(&type_, &i.type_) {
                            symbol_table.add(&i.name, &i.type_);
                        } else {
                            return Err(SemanticError { err_msg: format!("{}:{} ---> The type of your let expression init is inconsistent with the declared type!",i.position.0,i.position.1), });
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                },
                _ => {}
            }
        }
        return Ok(OBJECT.to_string());
    }
}

impl TypeChecker for Assignment {
    fn check_type(
        &self,
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
        return Err(SemanticError { err_msg: format!("{}:{} ---> Some semantic errors occurred in your Assignment! It may be because you have different types on both sides of the equal sign!",self.position.0,self.position.1) });
    }
}

impl TypeChecker for Math {
    fn check_type(
        &self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        let left_type = (*self.left).check_type(symbol_table, class_table);
        let right_type = (*self.right).check_type(symbol_table, class_table);
        let is_compute: bool;

        match *(self.op.clone()) {
            MathOp::ComputeOp(_) => is_compute = true,
            MathOp::CondOp(_) => is_compute = false,
        }
        match left_type {
            Ok(left) => match right_type {
                Ok(right) => {
                    if left == INT.to_string() && right == INT.to_string() {
                        if is_compute {
                            return Ok(INT.to_string());
                        } else {
                            return Ok(BOOL.to_string());
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
        &self,
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
                            self.postion.0, self.postion.1
                        ),
                    });
                }
            }
            Err(e) => return Err(e),
        }
        for then_expr in *(self.then_body.clone()) {
            let then_type = then_expr.check_type(symbol_table, class_table);
            match then_type {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        for else_expr in *(self.else_body.clone()) {
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
        &self,
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
                            self.postion.0, self.postion.1
                        ),
                    });
                }
            }
            Err(e) => return Err(e),
        }

        for body_expr in *(self.body.clone()) {
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
        &self,
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
