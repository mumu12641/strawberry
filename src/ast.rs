pub type Identifier = String;
pub type Type = String;
pub type Boolean = bool;
pub type Int = u32;
pub type Str = String;

#[derive(Debug)]
pub struct Program {
    pub classes: Vec<Class>,
}

#[derive(Debug)]
pub struct Class {
    pub name: Type,
    pub parent: Option<Type>,
    pub features: Vec<Feature>,
}
#[derive(Debug)]
pub enum Feature {
    Attribute(VarDecl),
    Method(MethodDecl),
}
#[derive(Debug)]
pub struct VarDecl {
    pub name: Identifier,
    pub type_: Type,
    pub init: Box<Option<Expr>>,
}
#[derive(Debug)]
pub struct MethodDecl {
    pub name: Identifier,
    pub param: Box<Vec<ParamDecl>>,
    pub return_type: Type,
    pub body: Box<Option<Vec<Expr>>>,
}

pub type ParamDecl = (Identifier, Type);

#[derive(Debug)]
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



#[derive(Debug)]
pub enum Expr {
    Identifier(Identifier),
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Identifier, Box<Expr>),
    Dispatch {
        target: Box<Option<Expr>>,
        fun_name: Identifier,
        actual:Box<Vec<Expr>>,
      

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
}
