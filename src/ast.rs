use std::hash::{Hash, Hasher};

pub type Identifier = String;
pub type Type = String;
pub type Boolean = bool;
pub type Int = u32;
pub type Str = String;

#[derive(Debug)]
pub struct Program {
    pub classes: Vec<Class>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Type,
    pub parent: Option<Type>,
    pub features: Vec<Feature>,
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

#[derive(Debug, Clone,PartialEq)]
pub enum Feature {
    Attribute(VarDecl),
    Method(MethodDecl),
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
        return self.name == other.name
            && self.type_ == other.type_
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

        // TODO: return type 
        return self.name == other.name
            && self.return_type == other.return_type
            && crate::util::do_vecs_match::<(String, String)>(&(*self.param), &(*other.param));
    }
}

pub type ParamDecl = (Identifier, Type);

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
    // pub fn get_name(){
    // }
}
