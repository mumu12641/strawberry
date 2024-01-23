use super::Position;

#[derive(Debug, Clone)]
pub enum Token {
    // raw type
    IntRawType(String),
    VoidRawType(String),
    BoolRawType(String),

    // keywords
    Class_(Position, String),
    Self_(String),
    Function(Position),
    Return(Position),
    If(Position),
    Else(Position),
    Let(Position),
    While(Position),
    For(Position),
    New(Position),
    Not(Position),
    Constructor(Position),
    Inherits,
    Isnull,
    Public,
    Private,
    Then,
    Import,
    From,
    ASM,

    // const and id and typeid
    StringConst(String),
    IntConst(String),
    BoolConst(bool),
    TypeId(String),
    Identifier(String, Position),

    // op
    Assign(Position),
    Arrow,
    Plus,
    Minus,
    Divide,
    Mul,
    Equal,
    More,
    MoreE,
    Less,
    LessE,

    // others
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Semicolon,
    Period,
    Comma,
    Colon,
    Newline,
    Whitespace(String),
    Comment,
    BlockComment(String),

    Error(String),
}
