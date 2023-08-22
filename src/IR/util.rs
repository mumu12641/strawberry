use std::collections::HashMap;

use super::abstract_present::{AbstractCode, AbstractInstruction};

impl AbstractCode {
    pub fn get_dest(&self) -> Option<String> {
        match self {
            AbstractCode::Instruction(instr) => match instr {
                AbstractInstruction::Constant { dest, .. } => return Some(dest.clone()),
                AbstractInstruction::Compute { dest, .. } => return Some(dest.clone()),
                AbstractInstruction::Assign { dest, .. } => return Some(dest.clone()),
                AbstractInstruction::Phi { dest, .. } => return Some(dest.clone()),
                _ => return None,
            },
            _ => return None,
        };
    }

    pub fn get_phi(&self) -> Option<&crate::IR::util::AbstractInstruction> {
        match self {
            AbstractCode::Instruction(instr) => match instr {
                AbstractInstruction::Phi { .. } => return Some(instr),
                _ => return None,
            },
            _ => return None,
        };
    }

    pub fn ssa_change_phi(
        &mut self,
        stack: &mut HashMap<&String, Vec<String>>,
        label_name: &String,
    ) {
        if let AbstractCode::Instruction(instr) = self {
            if let AbstractInstruction::Phi { args, labels, .. } = instr {

                if let Some(index) = labels.iter().position(|l| *l == *label_name) {
                    let var = args.get(index).unwrap();
                    if stack.get(var).unwrap().len() > 1 {
                        *args.get_mut(index).unwrap() =
                            stack.get(var).unwrap().last().unwrap().to_string();
                    } else {
                        *args.get_mut(index).unwrap() = "__undefined".to_string();
                    }
                } else {
                }
            }
        }
    }

    pub fn ssa_change_src(&mut self, stack: &mut HashMap<&String, Vec<String>>) {
        if let AbstractCode::Instruction(instr) = self {
            match instr {
                // AbstractInstruction::Constant { dest, const_type, value } => todo!(),
                AbstractInstruction::Compute { left, right, .. } => {
                    *left = stack.get(left).unwrap().last().unwrap().to_string();
                    *left = stack.get(right).unwrap().last().unwrap().to_string();
                }
                AbstractInstruction::Assign { src, .. } => {
                    *src = stack.get(src).unwrap().last().unwrap().to_string()
                }
                AbstractInstruction::Ret { src } => {
                    *src = stack.get(src).unwrap().last().unwrap().to_string()
                }
                AbstractInstruction::Br { arg, .. } => {
                    *arg = stack.get(arg).unwrap().last().unwrap().to_string()
                }
                // AbstractInstruction::Jmp { label } => todo!(),
                // AbstractInstruction::Phi { dest, labels, args } => todo!(),
                _ => {}
            }
        }
    }

    pub fn ssa_change_dest(
        &mut self,
        stack: &mut HashMap<&String, Vec<String>>,
        push_num: &mut HashMap<&String, usize>,
        var_env: &mut HashMap<&String, usize>,
    ) {
        match self {
            AbstractCode::Instruction(instr) => match instr {
                AbstractInstruction::Constant { dest, .. } => {
                    Self::ssa_handle(dest, stack, push_num, var_env)
                }
                AbstractInstruction::Compute { dest, .. } => {
                    Self::ssa_handle(dest, stack, push_num, var_env)
                }
                AbstractInstruction::Assign { dest, .. } => {
                    Self::ssa_handle(dest, stack, push_num, var_env)
                }
                AbstractInstruction::Phi { dest, .. } => {
                    Self::ssa_handle(dest, stack, push_num, var_env)
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn ssa_handle(
        dest: &mut String,
        stack: &mut HashMap<&String, Vec<String>>,
        push_num: &mut HashMap<&String, usize>,
        var_env: &mut HashMap<&String, usize>,
    ) {
        let num = var_env.get_mut(dest).unwrap();
        stack
            .get_mut(dest)
            .unwrap()
            .push(format!("{}.{}", dest, num));
        *num += 1;
        push_num.get_mut(dest).unwrap();

        *dest = stack.get(dest).unwrap().last().unwrap().to_string()
    }
}
