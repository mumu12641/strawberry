use crate::grammar::ast::expr::{Dispatch, Expr, Return};

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

            Expr::Identifier(e, _) => {
                code_generator.write(format!("# identifier :{}", e), true);
                let map = code_generator
                    .environment
                    .env
                    .get(&code_generator.environment.curr_class)
                    .unwrap();
                let location = map.find(e).unwrap();
                if location.offset == usize::MAX {
                    code_generator.write(format!("movq %rbx, %rax"), true);
                } else {
                    code_generator.write(
                        format!("movq {}({}), %rax", location.offset, location.reg),
                        true,
                    )
                }
                code_generator.write(format!(""), true);
            }

            Expr::Dispatch(e) => e.code_generate(code_generator),

            Expr::Return(e) => e.code_generate(code_generator),

            _ => {}
        }
    }
}

impl CodeGenerate for Return {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        // todo!()
        let e = *(self.val.clone());
        e.code_generate(code_generator);

        // code_generator.write(format!("movq 16(%rax), %rax"), true);

        // code_generator.write(format!("ret "), true);
        code_generator.method_end();
    }
}

impl CodeGenerate for Dispatch {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        // self.actual
        // self.fun_name
        // self.target

        // push actual
        for i in *self.actual.clone() {
            i.code_generate(code_generator);
            code_generator.write(format!("push %rax"), true);
        }
        // if let Some(expr_) = *(method.body.clone()) {
        if let Some(target) = *self.target.clone() {
            target.code_generate(code_generator);
            code_generator.write(format!("movq *16(%rax), %rdi"), true);
            code_generator.write(
                format!(
                    "call *{}(%rdi)",
                    code_generator
                        .dispatch_table
                        .get(&(
                            code_generator.environment.curr_class.to_string(),
                            self.fun_name.to_string(),
                        ))
                        .unwrap()
                ),
                true,
            );
        }
    }
}
