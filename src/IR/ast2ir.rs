use std::ops::{Deref, DerefMut};

use crate::{
    grammar::ast::{
        expr::{Expr, Let},
        Int,
    },
    INT,
};

use super::abstract_present::{AbstractCode, AbstractInstruction, Literal};

pub trait Ast2IR {
    fn ast2ir(&mut self, codes: &mut Vec<AbstractCode>);
}

// pub struct 

impl Ast2IR for Expr {
    fn ast2ir(&mut self, codes: &mut Vec<AbstractCode>) {
        match self {
            Expr::Let(e) => e.ast2ir(codes),
            Expr::Identifier(e) => {}
            _ => {}
        }
    }
}

impl Ast2IR for Let {
    fn ast2ir(&mut self, codes: &mut Vec<AbstractCode>) {
        // todo!()
        // self.var_decls
        for i in self.var_decls.deref_mut() {
            // i.init
            // i.name

            // println!("{:?}",codes);
            if let Some(expr) = i.init.deref_mut() {
                expr.ast2ir(codes);
            }
        }
    }
}
