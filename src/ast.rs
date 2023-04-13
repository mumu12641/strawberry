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
    pub feature: Vec<Feature>,
}
#[derive(Debug)]
pub enum Feature {
    Attribute(AttrDecl),
    Method(MethodDecl),
}
#[derive(Debug)]
pub struct AttrDecl {
    pub name: Identifier,
    pub type_: Type,
    pub init: Option<Expr>,
}
#[derive(Debug)]
pub struct MethodDecl {
    pub name:Identifier,
    pub param:Box<Vec<ParamDecl>>,
    pub return_type:Type,
    pub body:Vec<Expr>
}

pub type ParamDecl = (Identifier, Type);

#[derive(Debug)]
pub enum Expr {
    Identifier(Identifier),
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assign(Identifier, Box<Expr>),
    // Let_expr(Identifier, )
}
