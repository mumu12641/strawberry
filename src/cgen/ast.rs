use crate::grammar::ast::expr::Expr;

use super::cgen::CodeGenerator;

pub trait CodeGenerate {
    fn code_generate(&self, code_generator: &mut CodeGenerator);
}
impl CodeGenerate for Expr {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        match self {
            Expr::Int(const_) => {
                let index = code_generator
                    .int_const_table
                    .get(const_.to_string().as_str())
                    .unwrap();
                code_generator.write(format!("movq $int_const_{}, %rax", index), true);
            }
            Expr::Str(const_) => {
                let index = code_generator.str_const_table.get(const_.as_str()).unwrap();
                code_generator.write(format!("movq $str_const_{}, %rax", index), true);
            }

            Expr::Bool(const_) => {
                let index = if *const_ { 1 } else { 0 };
                code_generator.write(format!("movq $bool_const_{}, %rax", index), true);
            }

            _ => {}
        }
    }
}
