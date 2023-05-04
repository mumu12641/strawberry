use std::hash::{Hash, Hasher};

use super::{
    expr::{Expr, TypeChecker},
    Identifier, Type,
};

#[derive(Debug, Clone)]
pub struct Class<'a> {
    pub name: Type,
    pub parent: Option<Type>,
    pub features: Vec<Feature<'a>>,
    pub line_num: usize,
}

impl PartialEq for Class<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Class<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Eq for Class<'_> {}

#[derive(Debug, Clone, PartialEq)]
pub enum Feature<'a> {
    Attribute(VarDecl),
    Method(MethodDecl<'a>),
}

impl Feature<'_> {
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
pub struct MethodDecl <'a> {
    pub name: Identifier,
    pub param: Box<Vec<ParamDecl>>,
    pub return_type: Type,
    // pub body: Box<Option<Vec<Expr>>>,
    pub body: Box<Option<Vec<&'a dyn TypeChecker>>>,
}

impl PartialEq for MethodDecl<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}
impl MethodDecl<'_> {
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
