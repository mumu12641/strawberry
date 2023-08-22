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

    pub fn from_ssa(&mut self) {
        // # function input args
        // func_args = func.get('args', [])

        // blocks = block_map(list(form_blocks(func['instrs'])))

        // def add_id_instr(arg, label, instr):
        //     '''
        //     add `id` instruction in the block `label`
        //     '''
        //     # generate add_instr
        //     id_instr = dict()
        //     id_instr['op'] = 'id'
        //     id_instr['type'] = instr['type']
        //     id_instr['dest'] = instr['dest']
        //     id_instr['args'] = [arg]

        //     blocks[label].insert(-1, id_instr) # insert before the last instruction

        // for name, block in blocks.items():
        //     phi_node_idx = list()
        //     for idx, instr in enumerate(block):

        //         if instr.get('op', None) == 'phi':
        //             phi_node_idx.append(idx)
        //             for arg, label in zip(instr['args'], instr['labels']):
        //                 # print(f"name: {name}, dest: {instr['dest']}, arg: {arg}, label: {label}")
        //                 add_id_instr(arg, label, instr)

        //     # remove this phi node. Need to delete from bottom to top, otherwise there would be some index error.
        //     for phi_idx in sorted(phi_node_idx, reverse=True):
        //         del block[phi_idx]

        // # Assemble instructions
        // func['instrs'] = flatten(blocks.values())
        let blocks = self.function.blocks.clone();
        for (i, block) in blocks.iter().enumerate() {
            let mut phi_node_idx: Vec<usize> = vec![];
            for (index, code) in block.instrs.iter().enumerate() {
                // if is phi
                if let AbstractCode::Instruction(instr) = code {
                    // phi_node_idx.push(index);
                    if let AbstractInstruction::Phi { dest, labels, args } = instr {
                        println!("now phi idx is {}",index);
                        phi_node_idx.push(index);
                        for i in 0..labels.len() {
                            let label_pos = self
                                .function
                                .blocks
                                .iter()
                                .position(|b| b.name == labels[i])
                                .unwrap();
                            self.function
                                .blocks
                                .get_mut(label_pos)
                                .unwrap()
                                .instrs
                                .push(AbstractCode::Instruction(AbstractInstruction::Assign {
                                    src: args[i].clone(),
                                    dest: dest.clone(),
                                    type_: None,
                                }));
                            // self.function.blocks.get_mut(
                            //     self.function
                            //         .blocks
                            //         .iter()
                            //         .position(|b| b.name == labels.get(i).unwrap()),
                            // );
                        }
                    }
                }
                // remove phi_node_idx
            }
            for idx in phi_node_idx {
                // block
                self.function.blocks.get_mut(i).unwrap().instrs.remove(1);
            }
        }
    }

    fn rename_var(&mut self) {
        // stack[v] is a stack of variable names (for every variable v)
        let var_infos = self.get_var_info();
        let mut stack: HashMap<&String, Vec<String>> = HashMap::new();

        // to generate new var
        let mut var_env: HashMap<&String, usize> = HashMap::new();
        for var_info in &var_infos {
            stack.insert(var_info.0, vec![var_info.0.clone()]);
            var_env.insert(var_info.0, 0);
        }
        self.rename(0, &mut stack, &mut var_env);
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
