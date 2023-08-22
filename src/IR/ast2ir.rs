use std::{
    ops::{Deref, DerefMut},
    vec,
};

use crate::{
    grammar::ast::expr::{
        Assignment, Cond, ConstructorCall, Expr, IdentifierSrtuct, Let, Math, Return, While,
    },
    INT,
};

use super::abstract_present::{
    AbstractBasicBlock, AbstractCode, AbstractInstruction, AbstractType, Literal,
};

pub type Dest = String;

pub struct Ast2IREnv {
    pub naming_num: i32,
    pub branch_num: i32,
    pub curr_block: usize,
}

impl Ast2IREnv {
    pub fn update_var_env(&mut self) {
        self.naming_num += 1;
    }

    pub fn update_branch_env(&mut self, num: i32) {
        self.branch_num += num;
    }

    pub fn get_branch_num(&self) -> i32 {
        // self.update_branch_env();
        return self.branch_num;
    }

    pub fn get_env_var_name(&mut self) -> Dest {
        self.update_var_env();
        return format!("t{}", self.naming_num);
    }

    pub fn get_env_br_name(&mut self, num: i32) -> String {
        return format!(".branch{}", num);
    }

    pub fn get_curr_block<'a>(
        &'a mut self,
        blocks: &'a mut Vec<AbstractBasicBlock>,
    ) -> &mut AbstractBasicBlock {
        return blocks.get_mut(self.curr_block).unwrap();
    }

    pub fn update_block(
        &mut self,
        block: AbstractBasicBlock,
        blocks: &mut Vec<AbstractBasicBlock>,
    ) {
        self.curr_block += 1;
        blocks.push(block);
    }
}

pub trait Ast2IR {
    fn ast2ir(&mut self, blocks: &mut Vec<AbstractBasicBlock>, env: &mut Ast2IREnv)
        -> Option<Dest>;
}

// pub struct

impl Ast2IR for Expr {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        match self {
            Expr::Int(int) => {
                let dest = env.get_env_var_name();
                env.get_curr_block(blocks)
                    .instrs
                    .push(AbstractCode::Instruction(AbstractInstruction::Constant {
                        dest: dest.clone(),
                        const_type: AbstractType::Type(INT.to_string()),
                        // const_type:,
                        value: Literal::Int(*int),
                    }));
                return Some(dest);
            }

            Expr::Assignment(e) => e.ast2ir(blocks, env),

            Expr::New(e) => e.ast2ir(blocks, env),

            Expr::Math(e) => e.ast2ir(blocks, env),

            Expr::Let(e) => e.ast2ir(blocks, env),

            Expr::Identifier(e) => e.ast2ir(blocks, env),

            Expr::Return(e) => e.ast2ir(blocks, env),

            Expr::Cond(e) => e.ast2ir(blocks, env),

            Expr::While(e) => e.ast2ir(blocks, env),

            _ => None,
        }
    }
}

impl Ast2IR for Let {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        for i in self.var_decls.deref_mut() {
            if let Some(expr) = i.init.deref_mut() {
                let src = expr.ast2ir(blocks, env).unwrap();
                env.get_curr_block(blocks)
                    .instrs
                    .push(AbstractCode::Instruction(AbstractInstruction::Assign {
                        src,
                        dest: i.name.clone(),
                        type_: None,
                    }))
            }
        }
        None
    }
}

impl Ast2IR for Assignment {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        let src = self.compute.deref_mut().ast2ir(blocks, env).unwrap();
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Assign {
                src,
                dest: self.id.clone(),
                type_: None,
            }));
        None
    }
}

impl Ast2IR for Math {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        let dest_left = self.left.ast2ir(blocks, env).unwrap();
        let dest_right = self.right.ast2ir(blocks, env).unwrap();
        let dest = env.get_env_var_name();
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Compute {
                left: dest_left,
                dest: dest.clone(),
                right: dest_right,
                op: self.op.clone().deref().to_owned(),
                type_: self.type_.clone(),
                // type_:None
            }));
        return Some(dest);
    }
}

impl Ast2IR for IdentifierSrtuct {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        let dest = env.get_env_var_name();
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Assign {
                src: self.name.clone(),
                dest: dest.clone(),
                // type_: Some(AbstractType::Type(self.type_.clone())),
                type_: None,
            }));
        Some(dest)
    }
}

impl Ast2IR for Return {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        // self.val
        // let s = self.val;
        if let Some(e) = self.val.as_deref_mut() {
            let dest = e.ast2ir(blocks, env).unwrap();
            env.get_curr_block(blocks)
                .instrs
                .push(AbstractCode::Instruction(AbstractInstruction::Ret {
                    src: dest,
                }))
        }
        None
    }
}

impl Ast2IR for Cond {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        // get labels
        let condition = self.test.ast2ir(blocks, env).unwrap();
        let left = env.get_env_br_name(env.get_branch_num());
        let right = env.get_env_br_name(env.get_branch_num() + 1);
        let end = env.get_env_br_name(env.get_branch_num() + 2);
        env.update_branch_env(3);

        // br
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Br {
                arg: condition,
                true_label: left.clone(),
                false_label: right.clone(),
            }));

        // left
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Label { label: left });
        for expr in self.then_body.deref_mut() {
            expr.ast2ir(blocks, env);
        }
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Jmp {
                label: end.clone(),
            }));

        // right
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Label { label: right });
        for expr in self.else_body.deref_mut() {
            expr.ast2ir(blocks, env);
        }
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Jmp {
                label: end.clone(),
            }));

        // end
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Label { label: end });
        None
    }
}

impl Ast2IR for While {
    fn ast2ir(
        &mut self,
        blocks: &mut Vec<AbstractBasicBlock>,
        env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        // get labels
        let condition_br = env.get_env_br_name(env.get_branch_num());
        let body_br = env.get_env_br_name(env.get_branch_num() + 1);
        let end_br = env.get_env_br_name(env.get_branch_num() + 2);
        env.update_branch_env(3);

        if env.curr_block == 0 {
            // is the first block
            env.get_curr_block(blocks)
                .instrs
                .push(AbstractCode::Instruction(AbstractInstruction::Jmp {
                    label: condition_br.clone(),
                }))
        }

        // update and br
        env.update_block(
            AbstractBasicBlock {
                instrs: vec![],
                name: condition_br.clone(),
                successors: vec![],
            },
            blocks,
        );
        env.get_curr_block(blocks).instrs.push(AbstractCode::Label {
            label: condition_br.clone(),
        });

        // condition head
        let condition = self.test.deref_mut().ast2ir(blocks, env).unwrap();
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Br {
                arg: condition,
                true_label: body_br.clone(),
                false_label: end_br.clone(),
            }));

        // body
        env.update_block(
            AbstractBasicBlock {
                instrs: vec![],
                name: body_br.clone(),
                successors: vec![],
            },
            blocks,
        );
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Label { label: body_br });

        for expr in self.body.deref_mut() {
            expr.ast2ir(blocks, env);
        }
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Instruction(AbstractInstruction::Jmp {
                label: condition_br,
            }));

        // end
        env.update_block(
            AbstractBasicBlock {
                instrs: vec![],
                name: end_br.clone(),
                successors: vec![],
            },
            blocks,
        );
        env.get_curr_block(blocks)
            .instrs
            .push(AbstractCode::Label { label: end_br });
        None
    }
}

impl Ast2IR for ConstructorCall {
    fn ast2ir(
        &mut self,
        _blocks: &mut Vec<AbstractBasicBlock>,
        _env: &mut Ast2IREnv,
    ) -> Option<Dest> {
        todo!()
    }
}
