use super::lexer::Position;

#[derive(Debug, Clone)]
pub enum Token {
    Class_(Position, String),
    Public,
    Private,
    Self_(String),
    Function(Position),
    Return(Position),
    If(Position),
    Then,
    Else(Position),

    Inherits,
    Let(Position),
    While(Position),
    For(Position),
    New(Position),
    Isnull,
    Not(Position),

    StringConst(String),
    IntConst(String),
    BoolConst(bool),
    TypeId(String),
    Identifier(String, Position),

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

    Import,
    From,

    Error(String),

    // for inline asm
    ASM,
    // todo
    // case,
}
