extern crate plex;
use crate::{grammar::token::Token, table::Tables, EMPTY};

use plex::lexer;

lexer! {

    fn next_token(text:'a) -> Token;
    "class" => Token::Class_(EMPTY,"".to_string()),
    // "self" => Token::Self_,
    "function"=>Token::Function(EMPTY),
    "fun" => Token::Function(EMPTY),
    "fn" => Token::Function(EMPTY),
    "return"=>Token::Return(EMPTY),

    "if" => Token::If(EMPTY),
    "then" => Token::Then,
    "else" => Token::Else(EMPTY),
    "inherits" => Token::Inherits,
    "let" => Token::Let(EMPTY),
    "while" => Token::While(EMPTY),
    "new"=>Token::New(EMPTY),
    "isvoid" => Token::Isvoid,
    "!" => Token::Not,
    "true"=>Token::BoolConst(true),
    "false" => Token::BoolConst(false),

    "[A-Z][a-zA-Z0-9_]*"=>Token::TypeId(text.to_owned()),
    "[a-z][a-zA-Z0-9_]*"=>Token::Identifier(text.to_owned(),EMPTY),
    "[0-9]+" => Token::IntConst(text.to_owned()),
    r#""[^"]*""# => parse_string(text),

    "="=>Token::Assign(EMPTY),
    "->" => Token::Arrow,
    r"\+" => Token::Plus,
    "-" => Token::Minus,
    r"\*" => Token::Mul,
    "/" => Token::Divide,
    "==" => Token::Equal,
    ">" => Token::More,
    "=>" => Token::MoreE,
    "<" => Token::Less,
    "<=" => Token::LessE,

    r#"\n"# => Token::Newline,
    r#"[ \t\r]+"# => Token::Whitespace,
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::Comment,
    r#"//[^\n]*"# => Token::Comment,

    "{" => Token::Lbrace,
    "}" => Token::Rbrace,
    r"\(" => Token::Lparen,
    r"\)" => Token::Rparen,
    ";" => Token::Semicolon,
    r"\." => Token::Period,
    r"," => Token::Comma,
    ":" => Token::Colon,

    "." => Token::Error(format!("Unexpected character: {}", text.to_owned())),

}

fn parse_string(text: &str) -> Token {
    let mut s: String = String::from("");
    let mut flag = false;
    for c in (&text[1..text.len() - 1]).chars() {
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
    current_line: usize,
    offset: usize,

    remaining: &'a str,
    tables: &'a mut Tables,
    file_name: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, tables: &'a mut Tables, filename: &'a str) -> Lexer<'a> {
        Lexer {
            current_line: 1,
            offset: 0,
            // original: text,
            remaining: text,
            tables,
            file_name: filename,
        }
    }
}

#[derive(Debug)]
pub struct LexicalError {}

pub type LineNum = usize;
pub type Off = usize;
pub type Position = (usize, usize);

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(LineNum, Token, Off), LexicalError>;
    fn next(&mut self) -> Option<Result<(LineNum, Token, Off), LexicalError>> {
        loop {
            let tok = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                self.offset += self.remaining.len() - new_remaining.len();
                self.remaining = new_remaining;
                tok
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                Token::Newline => {
                    self.current_line += 1;
                    self.offset = 0;
                    continue;
                }
                Token::StringConst(text) => {
                    self.tables.string_table.insert(text.clone());
                    return Some(Ok((
                        self.current_line,
                        Token::StringConst(text),
                        self.offset,
                    )));
                }
                Token::IntConst(text) => {
                    self.tables.int_table.insert(text.clone());
                    return Some(Ok((self.current_line, Token::IntConst(text), self.offset)));
                }
                Token::Identifier(text, _) => {
                    self.tables.id_table.insert(text.clone());

                    return Some(Ok((
                        self.current_line,
                        Token::Identifier(text, (self.current_line, self.offset)),
                        self.offset,
                    )));
                }
                Token::TypeId(text) => {
                    self.tables.string_table.insert(text.clone());

                    return Some(Ok((self.current_line, Token::TypeId(text), self.offset)));
                }

                Token::Class_(_, _) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Class_((self.current_line, self.offset), self.file_name.to_string()),
                        self.offset,
                    )));
                }

                Token::Function(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Function((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Return(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Return((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::If(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::If((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Else(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Else((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Let(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Let((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::While(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::While((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::New(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::New((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Assign(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Assign((self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                token => {
                    return Some(Ok((self.current_line, token, self.offset)));
                }
            }
        }
    }
}
