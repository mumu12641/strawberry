
use crate::{lexer::Position, utils::util::do_vecs_match};
use std::collections::HashMap;
use std::{
    hash::{Hash, Hasher},
    io::SeekFrom,
    ops::Deref,
};

use super::{expr::Expr, Identifier, ParamDecl, Type, EMPTY_POSITION};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Type,
    pub parent: Option<Type>,
    pub features: Vec<Feature>,
    pub position: Position,
    pub file_name: String,
}
impl Eq for Class {}
impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Class {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    Attribute(VarDecl),
    Method(MethodDecl),
    Constructor(ConstructorDecl),
}

impl Feature {
    pub fn check_param(&self, other: &Self) -> bool {
        match self {
            Self::Method(m) => {
                return m.check_param(other);
            }
            _ => {}
        }
        return false;
    }

    pub fn check_return_type(&self, other: &Self) -> bool {
        match self {
            Self::Method(m) => {
                return m.check_return_type(other);
            }
            _ => {}
        }
        return false;
    }

    pub fn get_ownership(&self) -> &Ownership {
        match self {
            Self::Method(m) => return &m.ownership,
            Self::Attribute(a) => return &a.ownership,
            Self::Constructor(_) => return &Ownership::Public,
        }
    }

    pub fn get_position(&self) -> Position {
        if let Self::Method(m) = self {
            return m.position;
        } else {
            return *EMPTY_POSITION;
        }
    }

    pub fn get_param(&self) -> &Box<Vec<ParamDecl>> {
        match self {
            Feature::Method(method) => return &method.param,
            Feature::Constructor(constructor) => return &constructor.param,
            _ => todo!(),
        }
    }

    pub fn get_body(&self) -> &Box<Option<Vec<Expr>>> {
        match self {
            Feature::Method(method) => return &method.body,
            Feature::Constructor(constructor) => return &constructor.body,
            _ => todo!(),
        }
    }

    pub fn get_param_len(&self) -> i32 {
        match self {
            Feature::Method(method) => return method.param.deref().len() as i32,
            Feature::Constructor(constructor) => return constructor.param.deref().len() as i32,
            _ => return 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: Identifier,
    pub type_: Option<Type>,
    pub init: Box<Option<Expr>>,
    pub position: Position,
    pub ownership: Ownership,
    
}


impl PartialEq for VarDecl {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Ownership {
    Private,
    Public,
    Default,
}

#[derive(Debug, Clone)]
pub struct MethodDecl {
    pub name: Identifier,
    pub param: Box<Vec<ParamDecl>>,
    pub return_type: Type,
    pub body: Box<Option<Vec<Expr>>>,
    pub position: Position,
    pub ownership: Ownership,
}

impl PartialEq for MethodDecl {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}
impl MethodDecl {
    pub fn check_param(&self, other: &Feature) -> bool {
        if let Feature::Method(m) = other {
            return crate::utils::util::do_vecs_match::<(String, String)>(
                &(*self.param),
                &(m.param),
            );
        } else {
            return false;
        }
    }

    pub fn check_return_type(&self, other: &Feature) -> bool {
        if let Feature::Method(m) = other {
            return self.return_type == m.return_type;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodCall {
    pub fun_name: Identifier,
    pub actual: Box<Vec<Expr>>,
}

#[derive(Debug, Clone)]
pub struct ConstructorDecl {
    pub param: Box<Vec<ParamDecl>>,
    pub body: Box<Option<Vec<Expr>>>,
    pub position: Position,
}


impl PartialEq for ConstructorDecl {
    fn eq(&self, other: &Self) -> bool {
        let other_param = other.param.deref();
        let self_param = self.param.deref();
        return do_vecs_match(other_param, self_param);
    }
}
