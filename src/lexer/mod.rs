pub mod lexer;
pub mod token;

use lazy_static::lazy_static;

use crate::{utils::table::Tables, ctx::CompileContext};

use self::lexer::Lexer;

lazy_static! {
    static ref EMPTY_POSITION: Position = Position { row: 0, column: 0 };
}
pub type LineNum = usize;
pub type Off = usize;

#[derive(Debug, Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

pub fn lexer_parse<'a>(ctx: &mut CompileContext) -> Lexer<'a> {
    return Lexer::new(ctx);
}
