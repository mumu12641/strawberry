use lazy_static::lazy_static;

use crate::lexer::Position;
pub mod class;
pub mod expr;
pub mod program;

pub type Identifier = String;
pub type Type = String;
pub type Boolean = bool;
pub type Int = u64;
pub type Str = String;
pub type ParamDecl = (Identifier, Type);

lazy_static! {
    static ref EMPTY_POSITION: Position = Position { row: 0, column: 0 };
}
