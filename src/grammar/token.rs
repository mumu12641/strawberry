#[derive(Debug, Clone)]
pub enum Token {
    Class_(usize),
    Function,
    Return,
    If,
    Then,
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
    Whitespace,
    Comment,

    Error(String),
    // todo
    // case,
}
