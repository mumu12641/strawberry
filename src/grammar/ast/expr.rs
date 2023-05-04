use std::fmt::Debug;

use crate::{utils::table::SymbolTable, OBJECT};

use super::{class::VarDecl, Boolean, Identifier, Int, Str, Type};
use crate::utils::table::*;

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
pub enum Expr {
    Identifier(Identifier),
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Identifier, Box<Expr>),

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
    fn check_type(&self, symbol_table: &mut SymbolTable<Identifier, Type>) -> Type;
}
// impl Expr {
//     pub fn check_type(&self, symbol_table: &mut SymbolTable<Identifier, Type>) -> bool {
//         match self {
//             Expr::Let(e) => {
//                 for i in *(e.clone()) {
//                     symbol_table.add(&i.name, &i.type_);
//                 }
//             }
//             _ => {}
//         }
//         return true;
//     }
// }

impl TypeChecker for Expr {
    fn check_type(&self, symbol_table: &mut SymbolTable<Identifier, Type>) -> Type {
        println!("check expr");
        return OBJECT.to_string();
    }
}

impl TypeChecker for Dispatch {
    fn check_type(&self, symbol_table: &mut SymbolTable<Identifier, Type>) -> Type {
        return OBJECT.to_string();
    }
}

impl TypeChecker for Let {
    fn check_type(&self, symbol_table: &mut SymbolTable<Identifier, Type>) -> Type {
        println!("for let expr check ");
        for i in *(self.var_decls.clone()) {
            symbol_table.add(&i.name, &i.type_);
        }
        return OBJECT.to_string();
    }
}
