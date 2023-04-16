mod ast;
mod lexer;
mod token;

pub struct SemanticChecker {
    classes: Vec<Class>,
}

pub struct SemanticError {}

pub impl SemanticChecker {
    fn new(classes_: Vec<Class>) -> SemanticChecker {
        SemanticChecker { classes: classes_ }
    }
    fn check() -> Result<Vec<Class>, SemanticError> {
        // * install constants and basic classes.
        // * get all classes not just user defined but also include IO, Object and so on.
        // * check Main
        // * check inherit
        // * check attributes
        // * check method override
        // * check all expressions
    }
}
