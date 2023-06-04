use std::ops::Deref;

use crate::{
    grammar::ast::{
        expr::{
            Assignment, ComputeOp, Cond, CondOp, Dispatch, Expr, Let, Math, MathOp, Return, While,
        },
        Identifier, Type,
    },
    BOOL, INT, STRING,
};

use super::cgen::CodeGenerator;

impl Expr {
    pub fn get_var_num(&self) -> Vec<(Identifier, Type)> {
        let mut vec: Vec<(Identifier, Type)> = Vec::new();

        match self {
            Expr::Let(e) => {
                let decls = e.var_decls.deref();
                for decl_ in decls {
                    vec.push((decl_.name.clone(), decl_.type_.clone()));
                }
                return vec.clone();
            }
            Expr::Cond(e) => {
                // let mut num = 0;
                for then_ in e.then_body.deref() {
                    // num += then_.get_var_num();
                    vec.append(&mut then_.get_var_num());
                }
                for else_ in e.else_body.deref() {
                    // num += else_.get_var_num();
                    vec.append(&mut else_.get_var_num());
                }
                // return num;
                return vec.clone();
            }
            Expr::While(e) => {
                // let mut num = 0;
                for expr_ in e.body.deref() {
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

            Expr::Identifier(e) => {
                let map = code_generator
                    .environment
                    .env
                    .get(&code_generator.environment.curr_class)
                    .unwrap();
                let location = map.find(&e.name).unwrap();
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

            Expr::New(e) => {
                code_generator.write(format!("push ${}_prototype", e), true);
                code_generator.write(format!("call Object.malloc"), true);
                code_generator.write(format!("addq $8, %rsp"), true);
                code_generator.write(format!("call {}.init", e), true);

                // "   .globl main
                // main:
                //     pushq $Main_prototype
                //     call Object.malloc
                //     addq $8, %rsp
                //     movq %rax, %rbx
                //     call Main.init
                //     movq %rbx, %rax
                //     call Main.main
                //     movq 16(%rax), %rax
                //     ret "
            }

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
        let e = self.val.deref();
        e.code_generate(code_generator);

        // code_generator.method_end();
    }
}

impl CodeGenerate for Dispatch {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        let temp = code_generator.environment.curr_class.clone();
        for i in self.actual.deref() {
            i.code_generate(code_generator);
            code_generator.write(format!("push %rax"), true);
        }
        if let Some(target_) = self.target.deref() {
            // change curr_class to target
            target_.code_generate(code_generator);

            match &target_ {
                Expr::Identifier(e) => {
                    code_generator.environment.curr_class = e.type_.clone();
                }
                Expr::Dispatch(e) => {
                    code_generator.environment.curr_class = e.type_.clone();
                }
                Expr::Str(_) => code_generator.environment.curr_class = STRING.to_string(),
                Expr::Int(_) => code_generator.environment.curr_class = INT.to_string(),
                Expr::Bool(_) => code_generator.environment.curr_class = BOOL.to_string(),
                Expr::New(e) => code_generator.environment.curr_class = e.clone(),
                _ => {}
            }
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
        for _ in self.actual.deref() {
            code_generator.write(format!("addq $8, %rsp"), true);
        }
        code_generator.environment.curr_class = temp;
    }
}

impl CodeGenerate for Let {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        for decl_ in self.var_decls.deref() {
            // expr_.init.
            // for expr_ in decl_.init

            if let Some(expr_) = decl_.init.deref() {
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
        let expr = self.compute.deref();

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
        let left = self.left.deref();
        left.code_generate(code_generator);
        code_generator.write(format!("pushq %rax"), true);

        let right = self.right.deref();
        right.code_generate(code_generator);

        // %r10 is right, %r11 is left
        code_generator.write(format!("movq 16(%rax), %r10"), true);
        code_generator.write(format!("movq (%rsp), %r11"), true);
        code_generator.write(format!("movq 16(%r11), %r11"), true);
        code_generator.write(format!("addq $8, %rsp"), true);

        match self.op.deref() {
            MathOp::ComputeOp(op_) => {
                match op_ {
                    ComputeOp::Add => {
                        code_generator.write(format!("addq %r10, %r11"), true);
                    }
                    ComputeOp::Minus => {
                        code_generator.write(format!("subq %r10, %r11"), true);
                    }
                    ComputeOp::Mul => {
                        code_generator.write(format!("movq %r11, %rax"), true);
                        code_generator.write(format!("mulq %r10"), true);
                        code_generator.write(format!("movq %rax, %r11"), true);
                    }
                    ComputeOp::Divide => {
                        code_generator.write(format!("movq %r11, %rax"), true);
                        code_generator.write(format!("divq %r10"), true);
                        code_generator.write(format!("movq %rax, %r11"), true);
                    }
                };
                code_generator.write(format!("pushq %r11"), true);
                code_generator.write(format!("pushq $Int_prototype"), true);
                code_generator.write(format!("call Object.malloc"), true);
                code_generator.write(format!("addq $8, %rsp"), true);
                code_generator.write(format!("call Int.init"), true);
                code_generator.write(format!("movq (%rsp), %r11"), true);
                code_generator.write(format!("movq %r11, 16(%rax)"), true);

                code_generator.write(format!("addq $8, %rsp"), true);
            }
            MathOp::CondOp(op_) => {
                // sub
                // if true jmp then
                // else
                code_generator.write(format!("subq %r10, %r11"), true);
                match op_ {
                    CondOp::More => code_generator.write(
                        format!("ja label_{}", code_generator.environment.label),
                        true,
                    ),

                    CondOp::MoreE => code_generator.write(
                        format!("jae label_{}", code_generator.environment.label),
                        true,
                    ),

                    CondOp::Less => code_generator.write(
                        format!("jb label_{}", code_generator.environment.label),
                        true,
                    ),
                    CondOp::LessE => code_generator.write(
                        format!("jbe label_{}", code_generator.environment.label),
                        true,
                    ),

                    CondOp::Equal => code_generator.write(
                        format!("je label_{}", code_generator.environment.label),
                        true,
                    ),
                }
            }
        }
        // %r11 is the result
    }
}

impl CodeGenerate for Cond {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        code_generator.environment.label += 1;
        let label_then = code_generator.environment.label;

        // if jump to then
        // eval test
        // jmp -> label_0
        self.test.code_generate(code_generator);

        code_generator.environment.label += 1;
        let label_done = code_generator.environment.label;

        // else body
        for else_ in self.else_body.deref() {
            else_.code_generate(code_generator);
        }

        // jmp  label_1
        code_generator.write(format!("jmp label_{}", label_done), true);

        // label_0: then body
        code_generator.write(format!("label_{}:", label_then), false);
        for then in self.then_body.deref() {
            then.code_generate(code_generator);
        }

        //  done:
        code_generator.write(format!("label_{}:", label_done), false);
    }
}

impl CodeGenerate for While {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {

        // jmp test ->label_loop + 1
        // loop:    label_loop
        //      body
        // test:
        //      test.code
        //      goto loop

        // jmp to loop

        code_generator.environment.label += 1;
        let label_loop = code_generator.environment.label;

        code_generator.write(format!("# jmp to test"), true);
        code_generator.write(format!("jmp label_{}", label_loop + 1), true);

        code_generator.write(format!("label_{}:", label_loop), false);
        for body_ in self.body.deref() {
            body_.code_generate(code_generator);
        }

        code_generator.write(format!("label_{}:", label_loop + 1), false);
        self.test.code_generate(code_generator);
        code_generator.environment.label += 1;
    }
}
