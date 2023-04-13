#[derive(Debug,Clone)]
pub enum Token {
    Class_,
    Function,
    If,
    Else,

    Inherits,
    Let,
    While,
    New,
    Isvoid,
    Not,

    StringConst(String),
    IntConst(String),
    BoolConst(bool),
    TypeId(String),
    Identifier(String),

    Assign,

    Plus,
    Minus,
    Divide,
    Mul,
    Equal,

    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Semicolon,
    Period,
    Comma,
    Colon,

    Newline,
    Whitespace,
    Comment,

    Error(String),
    // todo
    // case,
}
