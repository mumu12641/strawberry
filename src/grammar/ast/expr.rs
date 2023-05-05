use std::{fmt::Debug, ops::DerefMut};

use crate::{
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
    Add,
    Minus,
    Mul,
    Divide,
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
}
#[derive(Debug, Clone)]
pub struct Cond {
    pub test: Box<Expr>,
    pub then_body: Box<Expr>,
    pub else_body: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct While {
    pub test: Box<Expr>,
    pub body: Box<Expr>,
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
}

#[derive(Debug, Clone)]
pub struct Let {
    pub var_decls: Box<Vec<VarDecl>>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: Identifier,
    pub compute: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Identifier),
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

            Expr::Identifier(e) => {
                println!("check id");
                if let Some(s) = symbol_table.find(e) {
                    println!("{}", s);
                    return Ok(s.clone());
                } else {
                    return Err(SemanticError {
                        err_msg: "The identifier does not exist or it has gone out of scope!"
                            .to_string(),
                    });
                }
            }
            Expr::Let(e) => return e.check_type(symbol_table, class_table),

            Expr::Assignment(e) => return e.check_type(symbol_table, class_table),

            Expr::Dispatch(e) => return e.check_type(symbol_table, class_table),

            Expr::Math(e) => return e.check_type(symbol_table, class_table),

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
        println!();
        println!("*****dispatch type check*****");
        println!("{}", &self.fun_name);
        match *(self.target.clone()) {
            Some(e) => {
                if let Ok(target_type) = e.check_type(symbol_table, class_table) {
                    if let Some(class) = class_table.get_classes().get(&target_type) {
                        println!("class name = {}", &class.name);
                        if let Some(v) = class_table.get_inheritance().get(&class.name) {
                            for class in v {
                                println!("current class is {}", &class.name);
                                for f in &class.features {
                                    if let Feature::Method(method) = f {
                                        println!("methd name is {}", &method.name);
                                        if &method.name == &self.fun_name {
                                            let method_param = *(method.param.clone());
                                            let actuals = *(self.actual.clone());
                                            if actuals.len() != method_param.len() {
                                                return Err(SemanticError { err_msg: "The actual number of parameters of your method call is not equal to the number of declared formal parameters".to_string(), });
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
                                                            return Err(SemanticError { err_msg:"The actual parameter type of your method call is not the same as the declared formal parameter type".to_string() });
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
        _class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        for i in *(self.var_decls.clone()) {
            symbol_table.add(&i.name, &i.type_);
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
        return Err(SemanticError { err_msg: "Some semantic errors occurred in your Assignment! It may be because you have different types on both sides of the equal sign! ".to_string() });
    }
}

impl TypeChecker for Math {
    fn check_type(
        &self,
        symbol_table: &mut SymbolTable<Identifier, Type>,
        class_table: &mut ClassTable,
    ) -> Result<Type, SemanticError> {
        println!();
        println!("*****check math *****");
        let left_type = (*self.left).check_type(symbol_table, class_table);
        let right_type = (*self.right).check_type(symbol_table, class_table);
        match left_type {
            Ok(left) => match right_type {
                Ok(right) => {
                    if left == INT.to_string() && right == INT.to_string() {
                        return Ok(INT.to_string());
                    } else {
                        return Err(SemanticError {
                        err_msg:
                            "The left and right sides of your mathematical operation are not all INT types!"
                                .to_string(),
                    });
                    }
                }
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }
}
