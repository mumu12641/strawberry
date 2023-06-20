use std::ops::Deref;

use crate::{
    grammar::ast::{
        expr::{
            Assignment, ComputeOp, Cond, CondOp, Dispatch, Expr, For, Isnull, Let, Math, MathOp,
            Not, Return, While,
        },
        Identifier, Type,
    },
    BOOL, INT, STRING,
};

use super::cgen::{CodeGenerator, Location};

impl Expr {
    pub fn get_var_num(&self) -> Vec<(Identifier, Type)> {
        let mut vec: Vec<(Identifier, Type)> = Vec::new();

        match self {
            Expr::Let(e) => {
                let decls = e.var_decls.deref();
                for decl_ in decls {
                    vec.push((decl_.name.clone(), decl_.type_.clone().unwrap()));
                }
                return vec.clone();
            }
            Expr::Cond(e) => {
                for then_ in e.then_body.deref() {
                    vec.append(&mut then_.get_var_num());
                }
                for else_ in e.else_body.deref() {
                    vec.append(&mut else_.get_var_num());
                }
                return vec.clone();
            }
            Expr::While(e) => {
                for expr_ in e.body.deref() {
                    vec.append(&mut expr_.get_var_num());
                }
                return vec.clone();
            }
            Expr::For(e) => {
                for init_ in e.init.deref() {
                    vec.append(&mut init_.get_var_num());
                }
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

            Expr::Not(e) => e.code_generate(code_generator),

            Expr::Isnull(e) => e.code_generate(code_generator),

            Expr::For(e) => e.code_generate(code_generator),

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
            code_generator.write(format!("cmpq $0, %rax"), true);
            code_generator.write(format!("je abort"), true);
            code_generator.write(format!("cmpq $0, 8(%rax)"), true);
            code_generator.write(format!("je abort"), true);
            code_generator.write(format!("movq 16(%rax), %rdi"), true);

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
            // println!(
            //     "in let {} code_generator.environment.var_offset = {}",
            //     decl_.name, code_generator.environment.var_offset
            // );
            code_generator
                .environment
                .env
                .get_mut(&code_generator.environment.curr_class)
                .unwrap()
                .add(
                    &decl_.name,
                    &Location {
                        reg: "%rbp".to_string(),
                        offset: -8 * (code_generator.environment.var_offset),
                        // type_: decl_.type_.clone(),
                    },
                );

            // let location = code_generator
            //     .environment
            //     .env
            //     .get_mut(&code_generator.environment.curr_class)
            //     .unwrap()
            //     .find(&decl_.name)
            //     .unwrap()
            //     .clone();

            if let Some(expr_) = decl_.init.deref() {
                expr_.code_generate(code_generator);
                code_generator.write(
                    format!(
                        "movq %rax, {}({})",
                        -8 * (code_generator.environment.var_offset),
                        "%rbp".to_string()
                    ),
                    true,
                );
            } else {
                code_generator.write(
                    format!(
                        "movq ${}_prototype, {}({})",
                        decl_.type_.clone().unwrap(),
                        -8 * (code_generator.environment.var_offset),
                        "%rbp".to_string()
                    ),
                    true,
                );
            }
            code_generator.environment.var_offset += 1;
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
                code_generator.write(format!("movq $1, %rdi"), true);
                code_generator.write(format!("movq $0, %rax"), true);
                code_generator.write(format!("subq %r10, %r11"), true);
                match op_ {
                    CondOp::More => code_generator.write(format!("cmova %rdi, %rax"), true),
                    CondOp::MoreE => code_generator.write(format!("cmovae %rdi, %rax"), true),
                    CondOp::Less => code_generator.write(format!("cmovb %rdi, %rax"), true),
                    CondOp::LessE => code_generator.write(format!("cmovbe %rdi, %rax"), true),
                    CondOp::Equal => code_generator.write(format!("cmove %rdi, %rax"), true),
                }
            }
        }
        // %r11 is the result
    }
}

impl CodeGenerate for Cond {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        let label_then = code_generator.environment.label + 1;
        let label_done = code_generator.environment.label + 2;
        code_generator.environment.label += 2;

        // if jump to then
        // eval test
        // jmp -> label_0
        self.test.code_generate(code_generator);
        match self.test.deref() {
            Expr::Math(_) => {}
            Expr::Not(_) => {}
            Expr::Isnull(_) => {}
            _ => {
                // else is bool type
                code_generator.write(format!("movq 16(%rax), %rax"), true);
            }
        }
        code_generator.write(format!("cmpq $1, %rax"), true);
        code_generator.write(format!("je label_{}", label_then), true);
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

        let label_loop = code_generator.environment.label + 1;
        let lable_done = label_loop + 1;
        code_generator.environment.label += 2;

        code_generator.write(format!("jmp label_{}", lable_done), true);

        code_generator.write(format!("label_{}:", label_loop), false);
        for body_ in self.body.deref() {
            body_.code_generate(code_generator);
        }

        code_generator.write(format!("label_{}:", lable_done), false);

        self.test.code_generate(code_generator);
        match self.test.deref() {
            Expr::Math(_) => {}
            Expr::Not(_) => {}
            Expr::Isnull(_) => {}
            _ => {
                // else is bool type
                code_generator.write(format!("movq 16(%rax), %rax"), true);
            }
        }
        code_generator.write(format!("cmpq $1, %rax"), true);
        code_generator.write(format!("je label_{}", label_loop), true);
    }
}

impl CodeGenerate for Not {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        self.expr.deref().code_generate(code_generator);
        code_generator.write(format!("movq 16(%rax), %rdi"), true);
        code_generator.write(format!("xor $1, %rdi"), true);
        code_generator.write(format!("movq %rdi, %rax"), true);
    }
}

impl CodeGenerate for Isnull {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        self.expr.deref().code_generate(code_generator);
        code_generator.write(format!("movq 8(%rax), %rax"), true);

        code_generator.write(format!("xor $1, %rax"), true);
        // code_generator.write(format!("movq %rdi, %rax"), true);
    }
}

impl CodeGenerate for For {
    fn code_generate(&self, code_generator: &mut CodeGenerator) {
        // todo!()

        // jmp test ->label_loop + 1
        // loop:    label_loop
        //      body
        // test:
        //      test.code
        //      goto loop
        code_generator
            .environment
            .env
            .get_mut(&code_generator.environment.curr_class)
            .unwrap()
            .enter_scope();

        let label_loop = code_generator.environment.label + 1;
        let lable_done = label_loop + 1;
        code_generator.environment.label += 2;

        for init_ in self.init.deref() {
            init_.code_generate(code_generator);
        }

        code_generator.write(format!("jmp label_{}", lable_done), true);

        code_generator.write(format!("label_{}:", label_loop), false);

        for body_ in self.body.deref() {
            body_.code_generate(code_generator);
        }

        for iter_ in self.iter.deref() {
            iter_.code_generate(code_generator);
        }

        code_generator.write(format!("label_{}:", lable_done), false);
        for test_ in self.test.deref() {
            test_.code_generate(code_generator);
            match test_ {
                Expr::Math(_) => {}
                Expr::Not(_) => {}
                Expr::Isnull(_) => {}
                _ => {
                    // else is bool type
                    code_generator.write(format!("movq 16(%rax), %rax"), true);
                }
            }
        }

        code_generator.write(format!("cmpq $1, %rax"), true);
        code_generator.write(format!("je label_{}", label_loop), true);

        code_generator
            .environment
            .env
            .get_mut(&code_generator.environment.curr_class)
            .unwrap()
            .exit_scope();
    }
}
