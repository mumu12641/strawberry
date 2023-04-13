extern crate plex;
use crate::token::Token;
use plex::lexer;

// Class_,
//     If,
//     Else,

//     Inherits,
//     Let,
//     While,
//     New,
//     Isvoid,
//     Not,

//     StringConst(String),
//     IntConst(String),
//     BoolConst(bool),
//     TypeId(String),
//     Identifier(String),

//     Assign,

//     Plus,
//     Minus,
//     Divide,
//     Mul,
//     Equal,

//     Whitespace,
//     Comment,

//     Error(String),
lexer! {

    fn next_token(text:'a) -> Token;
    "class" => Token::Class_,
    "function"=>Token::Function,
    "fun" => Token::Function,
    "fn" => Token::Function,

    "if" => Token::If,
    "else" => Token::Else,
    ":" => Token::Inherits,
    "let" => Token::Let,
    "while" => Token::While,
    "new"=>Token::New,
    "isvoid" => Token::Isvoid,
    "not" => Token::Not,
    "[A-Z][a-zA-Z0-9_]*"=>Token::TypeId(text.to_owned()),
    "[a-z][a-zA-Z0-9_]*"=>Token::Identifier(text.to_owned()),
    "true"=>Token::BoolConst(true),
    "false" => Token::BoolConst(false),
    "[0-9]+" => Token::IntConst(text.to_owned()),
    r#""[^"]*""# => parse_string(text),

    "="=>Token::Assign,
    r"\+" => Token::Plus,
    "-" => Token::Minus,
    r"\*" => Token::Mul,
    "/" => Token::Divide,
    "==" => Token::Equal,


    r#"[ \t\r\n]+"# => Token::Whitespace,
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::Comment,
    r#"//[^\n]*"# => Token::Comment,

    "{" => Token::Lbrace,
    "}" => Token::Rbrace,
    r"\(" => Token::Lparen,
    r"\)" => Token::Rparen,
    ";" => Token::Semicolon,
    "=" => Token::Equal,
    r"\." => Token::Period,
    r"," => Token::Comma,

    "." => Token::Error(format!("Unexpected character: {}", text.to_owned())),

}

fn parse_string(text: &str) -> Token {
    let mut s: String = String::from("");
    let mut flag = false;
    for c in text.chars() {
        if c == '\\' && !flag {
            flag = true;
            continue;
        }
        if flag {
            match c {
                'n' => s.push('\n'),
                't' => s.push('\t'),
                _ => s.push(c),
            }
            flag = false;
        } else {
            s.push(c);
        }
    }
    return Token::StringConst(s);
}

#[derive(Debug)]
pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        Lexer {
            original: text,
            remaining: text,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub len: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let start = self.original.len() - self.remaining.len();
                let len = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { start, len })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
