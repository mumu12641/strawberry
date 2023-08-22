use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::IR::abstract_present::{
    AbstractBasicBlock, AbstractCode, AbstractFunction, AbstractInstruction,
};

pub struct SSAPass {
    pub function: AbstractFunction,
    pub dom_frontier: HashMap<usize, HashSet<usize>>,
    pub dom_tree: HashMap<usize, HashSet<usize>>,
    pub pred: HashMap<usize, HashSet<usize>>,
}

impl SSAPass {
    pub fn to_ssa(&mut self) {
        //         for v in vars:
        //          for d in Defs[v]:  # Blocks where v is assigned.
        //              for block in DF[d]:  # Dominance frontier.
        //                  Add a ϕ-node to block,
        //                      unless we have done so already.
        //                  Add block to Defs[v] (because it now writes to v!),
        //                      unless it's already in there.

        // first: insert phi node
        self.insert_phi_node();

        // second: rename var
        self.rename_var();
    }

    fn rename_var(&mut self) {
        // stack[v] is a stack of variable names (for every variable v)
        let var_infos = self.get_var_info();
        let mut stack: HashMap<&String, Vec<String>> = HashMap::new();

        // to generate new var
        let mut var_env: HashMap<&String, usize> = HashMap::new();
        for var_info in &var_infos {
            // stack.push(var_info.0);
            stack.insert(var_info.0, vec![var_info.0.clone()]);
            var_env.insert(var_info.0, 0);
        }
        self.rename(0, &mut stack, &mut var_env);
        // def rename(block):
        // for instr in block:
        //   replace each argument to instr with stack[old name]
        //   replace instr's destination with a new name
        //   push that new name onto stack[old name]

        // for s in block's successors:
        //   for p in s's ϕ-nodes:
        //     Assuming p is for a variable v, make it read from stack[v].

        // for b in blocks immediately dominated by block:
        //   # That is, children in the dominance tree.
        //   rename(b)

        // pop all the names we just pushed onto the stacks

        // rename(entry)
    }
    fn rename(
        &mut self,
        index: usize,
        stack: &mut HashMap<&String, Vec<String>>,
        var_env: &mut HashMap<&String, usize>,
    ) {
        println!("");
        println!("***********************");

        println!("var env is {:?}", var_env);
        // println!("")

        let block_name = self.function.blocks.get_mut(index).unwrap().name.clone();
        println!("now block is {}", &block_name);
        let mut push_num: HashMap<&String, usize> = HashMap::new();
        let successors = self.function.blocks.get(index).unwrap().successors.clone();
        for key in var_env.keys() {
            push_num.insert(*key, 0);
        }
        for code in &mut self.function.blocks.get_mut(index).unwrap().instrs {
            code.ssa_change_src(stack);
            code.ssa_change_dest(stack, &mut push_num, var_env);
        }

        for succ in successors {
            println!("now succ is {}", succ);
            let block = self.function.blocks.get_mut(succ).unwrap();

            for code in &mut block.instrs {
                code.ssa_change_phi(stack, &block_name);
            }
        }
        println!("{}", self.function);
        for tree_index in self.dom_tree.get(&index).unwrap().clone() {
            self.rename(tree_index, stack, var_env);
        }
        for num in push_num.iter() {
            for _ in 0..*num.1 {
                stack.get_mut(*num.0).unwrap().pop();
            }
        }
    }

    fn insert_phi_node(&mut self) {
        let mut phi_already: HashMap<String, HashSet<usize>> = HashMap::new();
        let var_infos = self.get_var_info();
        for var_info in var_infos.iter() {
            phi_already.insert(var_info.0.to_string(), HashSet::new());
        }
        for var_info in var_infos.iter() {
            for def in var_info.1 {
                for block in self.dom_frontier.get(def).unwrap() {
                    if !phi_already.get(var_info.0).unwrap().contains(block) {
                        // get block's pred label name
                        let labels_block: Vec<AbstractBasicBlock> = self
                            .function
                            .blocks
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| self.pred.get(block).unwrap().contains(i))
                            .map(|b| b.1.clone())
                            .collect();
                        let len = labels_block.len();
                        self.function.blocks.get_mut(*block).unwrap().instrs.insert(
                            1,
                            AbstractCode::Instruction(AbstractInstruction::Phi {
                                dest: var_info.0.clone(),
                                labels: labels_block.iter().map(|b| b.name.clone()).collect(),
                                args: vec![var_info.0.clone(); len],
                            }),
                        );
                    }
                }
            }
        }
    }
    fn get_var_info(&self) -> HashMap<String, HashSet<usize>> {
        let mut var_info: HashMap<String, HashSet<usize>> = HashMap::new();
        for arg in &self.function.args {
            var_info.insert(arg.name.clone(), HashSet::from_iter(vec![0]));
        }

        for (index, block) in self.function.blocks.iter().enumerate() {
            for instr in &block.instrs {
                let dest = instr.get_dest();

                if let Some(dest_) = dest {
                    if !var_info.contains_key(&dest_) {
                        var_info.insert(dest_, HashSet::from_iter(vec![index]));
                    } else {
                        var_info.get_mut(&dest_).unwrap().insert(index);
                    }
                }
            }
        }
        println!("var info is {:?}", &var_info);
        return var_info;
    }
}
