use super::{class::Class, expr::Import};
#[allow(dead_code)]
pub struct Program {
    pub impots: Vec<Import>,
    pub classes: Vec<Class>,
}
