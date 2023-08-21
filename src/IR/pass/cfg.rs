use crate::IR::abstract_present::{AbstractCode, AbstractFunction, AbstractInstruction};

/// build cfg for every function
pub struct CFG {
    pub function: AbstractFunction,
}
impl CFG {
    pub fn create_cfg(&mut self) {
        // self.function.blocks
        let blocks = self.function.blocks.clone();
        for block in &mut self.function.blocks {
            if let Some(last) = block.instrs.last() {
                // if let
                if let AbstractCode::Instruction(instr) = last {
                    match instr {
                        AbstractInstruction::Br {
                            true_label,
                            false_label,
                            ..
                        } => {
                            block
                                .successors
                                .push(blocks.iter().position(|b| b.name == *true_label).unwrap());
                            block
                                .successors
                                .push(blocks.iter().position(|b| b.name == *false_label).unwrap());
                        }
                        AbstractInstruction::Jmp { label } => block
                            .successors
                            .push(blocks.iter().position(|b| b.name == *label).unwrap()),
                        _ => {
                            let index = blocks.iter().position(|b| b.name == block.name).unwrap();
                            if index != blocks.len() - 1 {
                                block.successors.push(index + 1);
                            }
                        }
                    }
                }
            }
        }
    }
}
