use crate::utils::table::SymbolTable;

use super::{class::VarDecl, Boolean, Identifier, Int, Str, Type};

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
pub enum Expr {
    Identifier(Identifier),
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Identifier, Box<Expr>),
    Dispatch {
        target: Box<Option<Expr>>,
        fun_name: Identifier,
        actual: Box<Vec<Expr>>,
    },
    Cond {
        test: Box<Expr>,
        then_body: Box<Expr>,
        else_body: Box<Expr>,
    },
    While {
        test: Box<Expr>,
        body: Box<Expr>,
    },
    Block(Box<Vec<Expr>>),
    Let(Box<Vec<VarDecl>>),
    New(Type),
    Isvoid(Box<Expr>),

    Math {
        left: Box<Expr>,
        op: Box<MathOp>,
        right: Box<Expr>,
    },

    Return {
        val: Box<Expr>,
    },
}

impl Expr {
    pub fn check_type(&self, symbol_table: &mut SymbolTable<Identifier, Type>) -> bool {
        match self {
            Expr::Let(e) => {
                for i in *(e.clone()) {
                    symbol_table.add(&i.name, &i.type_);
                }
            }
            _ => {}
        }
        return true;
    }
}
