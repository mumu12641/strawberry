use std::hash::{Hash, Hasher};

use super::{Type, Identifier, expr::Expr};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Type,
    pub parent: Option<Type>,
    pub features: Vec<Feature>,
    pub line_num: usize,
}

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
impl Eq for Class {}

#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    Attribute(VarDecl),
    Method(MethodDecl),
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
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: Identifier,
    pub type_: Type,
    pub init: Box<Option<Expr>>,
}

impl PartialEq for VarDecl {
    fn eq(&self, other: &Self) -> bool {
        // TODO: type_
        return self.name == other.name && self.type_ == other.type_;
    }
}

#[derive(Debug, Clone)]
pub struct MethodDecl {
    pub name: Identifier,
    pub param: Box<Vec<ParamDecl>>,
    pub return_type: Type,
    pub body: Box<Option<Vec<Expr>>>,
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
        // TODO: return type
        if let Feature::Method(m) = other {
            return self.return_type == m.return_type;
        } else {
            return false;
        }
    }
}

pub type ParamDecl = (Identifier, Type);