use crate::grammar::{
    ast::expr::{Assignment, ComputeOp, Cond, Dispatch, Expr, Let, Math, MathOp, Return, While},
};

use super::cgen::CodeGenerator;

impl Expr {
    pub fn get_var_num(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        match self {
            Expr::Let(e) => {
                let decls = *(e.var_decls.clone());
                for decl_ in decls {
                    vec.push(decl_.name);
                }
                return vec.clone();
            }
            Expr::Cond(e) => {
                // let mut num = 0;
                for then_ in *(e.then_body.clone()) {
                    // num += then_.get_var_num();
                    vec.append(&mut then_.get_var_num());
                }
                for else_ in *(e.else_body.clone()) {
                    // num += else_.get_var_num();
                    vec.append(&mut else_.get_var_num());
                }
                // return num;
                return vec.clone();
            }
            Expr::While(e) => {
                // let mut num = 0;
                for expr_ in *(e.body.clone()) {
                    // num += expr_.get_var_num();
                    vec.append(&mut expr_.get_var_num());
                }
                // return num;
                return vec.clone();
            }
            _ => {
                return vec![];
            }
        }
    }
}

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
                let map = code_generator
                    .environment
                    .env
                    .get(&code_generator.environment.curr_class)
                    .unwrap();
                let location = map.find(e).unwrap();
                if location.offset == i32::MAX {
                    code_generator.write(format!("movq %rbx, %rax"), true);
                } else {
                    code_generator.write(
                        format!("movq {}({}), %rax", location.offset, location.reg),
                        true,
                    )
                }
                code_generator.write(format!(""), true);
            }

            Expr::New(e) => {}

            Expr::Dispatch(e) => e.code_generate(code_generator),

            Expr::Return(e) => e.code_generate(code_generator),

            Expr::Let(e) => e.code_generate(code_generator),

            Expr::Assignment(e) => e.code_generate(code_generator),

            Expr::Math(e) => e.code_generate(code_generator),

            Expr::Cond(e) => e.code_generate(code_generator),

            Expr::While(e) => e.code_generate(code_generator),

            _ => {}
        }
    }
}

impl CodeGenerate for Return {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        // todo!()
        let e = *(self.val.clone());
        e.code_generate(code_generator);

        // code_generator.method_end();
    }
}

impl CodeGenerate for Dispatch {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        for i in *self.actual.clone() {
            i.code_generate(code_generator);
            code_generator.write(format!("push %rax"), true);
        }
        if let Some(target) = *self.target.clone() {
            target.code_generate(code_generator);
            code_generator.write(format!("movq 8(%rax), %rdi"), true);
            // TODO: the class might be NULL
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
        for _ in *self.actual.clone() {
            code_generator.write(format!("addq $8, %rsp"), true);
        }
    }
}

impl CodeGenerate for Let {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        for decl_ in *(self.var_decls.clone()) {
            // expr_.init.
            // for expr_ in decl_.init

            if let Some(expr_) = *(decl_.init.clone()) {
                expr_.code_generate(code_generator);
                // decl_.name
                let location = code_generator
                    .environment
                    .env
                    .get_mut(&code_generator.environment.curr_class)
                    .unwrap()
                    .find(&decl_.name)
                    .unwrap()
                    .clone();
                code_generator.write(
                    format!("movq %rax, {}({})", location.offset, location.reg),
                    true,
                );
            }
        }
    }
}

impl CodeGenerate for Assignment {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        let expr = *self.compute.clone();

        let location = code_generator
            .environment
            .env
            .get_mut(&code_generator.environment.curr_class)
            .unwrap()
            .find(&self.id)
            .unwrap()
            .clone();

        expr.code_generate(code_generator);
        code_generator.write(
            format!("movq %rax, {}({})", location.offset, location.reg),
            true,
        );
    }

}

impl CodeGenerate for Math {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {

        // r10-r11 for temp register
        let left = *self.left.clone();
        left.code_generate(code_generator);
        code_generator.write(format!("pushq %rax"), true);

        let right = *self.right.clone();
        right.code_generate(code_generator);

        code_generator.write(format!("movq 16(%rax), %r10"), true);
        code_generator.write(format!("movq (%rsp), %r11"), true);
        code_generator.write(format!("movq 16(%r11), %r11"), true);

        match *self.op.clone() {
            MathOp::ComputeOp(op_) => match op_ {
                ComputeOp::Add => {
                    code_generator.write(format!("addq %r10, %r11"), true);
                    code_generator.write(format!("pushq %r11"), true);
                    // %r11 is the result
                    code_generator.write(
                        format!(
                            "
                pushq $Int_prototype
                call Object.malloc
                addq $8, %rsp`
                call Int.init
                movq (%rsp), %r11
                movq %r11, 16(%rax)
                "
                        ),
                        true,
                    );
                    code_generator.write(format!("addq $8, %rsp"), true);
                }
                _ => {}
            },
            MathOp::CondOp(_) => todo!(),
        }

        code_generator.write(format!("addq $8, %rsp"), true);
    }
}

impl CodeGenerate for Cond {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        todo!()
    }
}

impl CodeGenerate for While {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        todo!()
    }
}
