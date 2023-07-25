use std::fmt::Debug;

use crate::{
    grammar::lexer::Position,
    semantic::semantic::SemanticError,
    utils::table::{ClassTable, SymbolTable},
    BOOL, INT, OBJECT, STRING,
};

use super::{
    class::{MethodCall, VarDecl},
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
pub enum DispatchExpr {
    Field(Identifier),
    Method(MethodCall),
}

#[derive(Debug, Clone)]
pub struct Dispatch {
    pub target: Box<Expr>,
    // pub fun_name: Identifier,
    // pub actual: Box<Vec<Expr>>,
    pub expr: DispatchExpr,
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
    pub val: Option<Box<Expr>>,
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
pub struct Import {
    pub file_name: String,
    pub class_name: Type,
}
#[derive(Debug, Clone)]
pub struct ConstructorCall {
    pub class_name: String,
    pub param: Option<Box<Vec<Expr>>>,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(IdentifierSrtuct),
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Assignment),

    ASM(String),

    Dispatch(Dispatch),
    Cond(Cond),
    While(While),
    For(For),
    Block(Box<Vec<Expr>>),
    Let(Let),
    New(ConstructorCall),
    Self_(Self_),
    Isvoid(Box<Expr>),

    Math(Math),
    Not(Not),
    Isnull(Isnull),

    Return(Return),

    Import(Import),
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
            Expr::New(constructor_call) => return constructor_call.class_name.clone(),
            Expr::Identifier(e) => return e.type_.clone(),
            Expr::Dispatch(e) => return e.type_.clone(),
            Expr::Self_(e) => return e.type_.clone(),
            Expr::Math(e) => return e.type_.clone(),
            _ => return OBJECT.to_string(),
        }
    }
}

impl Expr {
    pub fn is_self_expr(&self) -> bool {
        if let Expr::Self_(_) = self {
            return true;
        } else {
            return false;
        }
    }
}
