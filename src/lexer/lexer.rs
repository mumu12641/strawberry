extern crate plex;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
};

use crate::{ctx::CompileContext, lexer::EMPTY_POSITION, table::Tables};

use plex::lexer;

use super::{token::Token, LineNum, Off, Position};

lexer! {

    fn next_token(text:'a) -> Token;

    //* keywords */
    "int" => Token::IntRawType(text.to_owned()),
    "void" => Token::VoidRawType(text.to_owned()),
    // "bool" => Token::BoolRawType(text.to_owned()),
    "str" => Token::StrRawType(text.to_owned()),

    "class" => Token::Class_(*EMPTY_POSITION,"".to_string()),
    "public" => Token::Public,
    "private" => Token::Private,
    "self" => Token::Self_(text.to_owned()),
    "function"=>Token::Function(*EMPTY_POSITION),
    "fun" => Token::Function(*EMPTY_POSITION),
    "fn" => Token::Function(*EMPTY_POSITION),
    "return"=>Token::Return(*EMPTY_POSITION),
    "import"=>Token::Import,
    "from"=>Token::From,
    "if" => Token::If(*EMPTY_POSITION),
    "then" => Token::Then,
    "else" => Token::Else(*EMPTY_POSITION),
    "inherits" => Token::Inherits,
    "let" => Token::Let(*EMPTY_POSITION),
    "while" => Token::While(*EMPTY_POSITION),
    "for" => Token::For(*EMPTY_POSITION),
    "new"=>Token::New(*EMPTY_POSITION),
    "null" => Token::Isnull,
    "!" => Token::Not(*EMPTY_POSITION),
    "true"=>Token::BoolConst(true),
    "false" => Token::BoolConst(false),
    "__asm__" => Token::ASM,
    "constructor" => Token::Constructor(*EMPTY_POSITION),


    //* const and id and typeid */
    "[A-Z][a-zA-Z0-9_]*"=>Token::TypeId(text.to_owned()),
    "[a-z][a-zA-Z0-9_]*"=>Token::Identifier(text.to_owned(),*EMPTY_POSITION),
    "[-]*[0-9]+" => Token::IntConst(text.to_owned()),
    r#""[^"]*""# => parse_string(text),

    //* op */
    "="=>Token::Assign(*EMPTY_POSITION),
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

    //* others */
    r#"\n"# => Token::Newline,
    r#"[ ]+"# => Token::Whitespace(text.to_owned()),
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::BlockComment(text.to_owned()),
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
    // tables: &'a mut Tables,
    // file_name: &'a str,
    pub ctx: &'a RefCell<CompileContext>,
    asm_flag: bool,
}

impl<'a> Lexer<'a> {
    // impl Lexer {
    pub fn new(ctx: &'a RefCell<CompileContext>) -> Lexer<'a> {
        Lexer {
            current_line: 1,
            offset: 0,
            remaining: "",
            // tables: &mut ctx.tables,
            // file_name: &ctx.file_name,
            ctx,
            asm_flag: false,
        }
    }
}

#[derive(Debug)]
pub struct LexicalError {}

impl<'a> Iterator for Lexer<'a> {
    // impl Iterator for Lexer {
    type Item = Result<(LineNum, Token, Off), LexicalError>;
    fn next(&mut self) -> Option<Result<(LineNum, Token, Off), LexicalError>> {
        loop {
            // self.remaining = &self.ctx.clone().borrow().content;
            // let tok = if let Some((tok, new_remaining)) = next_token(&self.ctx.borrow().content) {
            //     self.offset +=
            //         self.ctx.borrow().content.chars().count() - new_remaining.chars().count();
            //     self.ctx.borrow_mut().content = new_remaining.to_owned();
            //     tok
            // } else {
            //     return None;
            // };
            let file_name = self.ctx.borrow().file_name.clone();
            let mut borrow_mut = self.ctx.borrow_mut();

            let tok = if let Some((tok, new_remaining)) = next_token(&borrow_mut.content) {
                self.offset += borrow_mut.content.chars().count() - new_remaining.chars().count();
                borrow_mut.content = new_remaining.to_owned();
                tok
            } else {
                return None;
            };
            match tok {
                Token::Comment => continue,

                Token::Whitespace(_) => continue,

                Token::BlockComment(s) => self.current_line += s.lines().count() - 1,

                Token::Newline => {
                    self.current_line += 1;
                    self.offset = 0;
                    continue;
                }

                Token::ASM => {
                    self.asm_flag = true;
                    return Some(Ok((self.current_line, Token::ASM, self.offset)));
                }

                Token::StringConst(text) => {
                    if self.asm_flag == false {
                        // self.ctx
                        //     .borrow_mut()
                        //     .tables
                        //     .string_table
                        //     .insert(text.clone());
                        borrow_mut.tables.string_table.insert(text.clone());
                    } else {
                        self.asm_flag = false;
                    }
                    return Some(Ok((
                        self.current_line,
                        Token::StringConst(text),
                        self.offset,
                    )));
                }
                Token::IntConst(text) => {
                    // self.ctx.borrow_mut().tables.int_table.insert(text.clone());
                    borrow_mut.tables.int_table.insert(text.clone());
                    return Some(Ok((self.current_line, Token::IntConst(text), self.offset)));
                }
                Token::Identifier(text, _) => {
                    // self.ctx.borrow_mut().tables.id_table.insert(text.clone());
                    borrow_mut.tables.id_table.insert(text.clone());

                    return Some(Ok((
                        self.current_line,
                        Token::Identifier(text, Position::new(self.current_line, self.offset)),
                        self.offset,
                    )));
                }
                Token::TypeId(text) => {
                    // self.ctx
                    //     .borrow_mut()
                    //     .tables
                    //     .string_table
                    //     .insert(text.clone());
                    borrow_mut.tables.string_table.insert(text.clone());

                    return Some(Ok((self.current_line, Token::TypeId(text), self.offset)));
                }

                Token::Class_(_, _) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Class_(
                            Position::new(self.current_line, self.offset),
                            // self.ctx.borrow().file_name.to_string(),
                            file_name,
                        ),
                        self.offset,
                    )));
                }

                Token::Function(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Function(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Return(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Return(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::If(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::If(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Else(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Else(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Let(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Let(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::While(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::While(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::New(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::New(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Assign(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Assign(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Not(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Not(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::For(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::For(Position::new(self.current_line, self.offset)),
                        self.offset,
                    )))
                }

                Token::Constructor(_) => {
                    return Some(Ok((
                        self.current_line,
                        Token::Constructor(Position::new(self.current_line, self.offset)),
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
