use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    slice::SliceIndex,
};

use crate::IR::abstract_present::AbstractFunction;

pub struct DOM {
    pub function: AbstractFunction,
    pub dom: HashMap<usize, HashSet<usize>>,
    pub dom_tree: HashMap<usize, HashSet<usize>>,
    pub dom_frontier: HashMap<usize, HashSet<usize>>,
    pub preds: HashMap<usize, HashSet<usize>>,
}
impl DOM {
    pub fn new(function_: AbstractFunction) -> DOM {
        let mut dom = DOM {
            function: function_,
            dom: HashMap::new(),
            dom_tree: HashMap::new(),
            dom_frontier: HashMap::new(),
            preds: HashMap::new(),
        };
        dom.get_pred();
        dom.get_dom();
        dom.get_dom_tree();
        dom.get_dom_frontier();

        dom
    }
    pub fn get_dom(&mut self) {
        // let mut dom: HashMap<usize, HashSet<usize>> = HashMap::new();
        // init every block -> all block
        // let preds = self.get_pred();
        let size = self.function.blocks.len();
        for i in 0..size {
            if i == 0 {
                self.dom.insert(i, (0..1).collect());
            } else {
                self.dom.insert(i, (0..size).collect());
            }
        }
        let mut changed = true;
        while changed {
            changed = false;

            // for every block
            for index in 1..size {
                let mut pred_intersect: HashSet<usize> = HashSet::new();
                let pred = self.preds.get(&index).unwrap();
                // get its all pred
                for (i, k) in pred.iter().enumerate() {
                    if i == 0 {
                        pred_intersect = self.dom.get(k).unwrap().clone();
                    } else {
                        pred_intersect = &pred_intersect & &self.dom.get(k).unwrap();
                    }
                }
                let new_dom: HashSet<usize> = &pred_intersect | &HashSet::from_iter(vec![index]);

                if new_dom != self.dom.get(&index).unwrap().clone() {
                    changed = true;
                    self.dom.insert(index, new_dom);
                }
            }
            if !changed {
                break;
            }
        }
        println!("dom is {:?}", &self.dom)
    }

    pub fn get_dom_frontier(&mut self) {
        // init is empty
        for i in 0..self.function.blocks.len() {
            self.dom_frontier.insert(i, HashSet::new());
        }

        let mut strict_dom = self.dom.clone();
        // delete self
        for set in strict_dom.iter_mut() {
            set.1.remove(&set.0);
        }
        // let pred_map = self.get_pred();
        for doms in self.dom.iter() {
            let succ_list = &self.function.blocks.get(*doms.0).unwrap().successors;
            for dom_label in doms.1 {
                for succ in succ_list {
                    // B A C
                    // C is B's succ, if A dom B but A do not dom C then C(succ) is A(dom_label)'s df
                    if !strict_dom[succ].contains(dom_label) {
                        self.dom_frontier.get_mut(dom_label).unwrap().insert(*succ);
                    }
                }
            }
        }
        println!("df is {:?}", &self.dom_frontier)
    }

    pub fn get_dom_tree(&mut self) {
        // Immediate Dominator
        // init is empty
        for i in 0..self.function.blocks.len() {
            self.dom_tree.insert(i, HashSet::new());
        }

        let mut strict_dom = self.dom.clone();
        // delete self
        for set in strict_dom.iter_mut() {
            set.1.remove(&set.0);
        }

        for doms in strict_dom.iter() {
            // B is set.0

            for dom_label in doms.1.iter() {
                // A is dom_label       A dom B
                let mut flag = true;
                for strict_dom_label in strict_dom.get(doms.0).unwrap() {
                    // C dom B
                    // if A dom C indicates A dom C then dom B
                    if strict_dom
                        .get(strict_dom_label)
                        .unwrap()
                        .contains(dom_label)
                    {
                        flag = false;
                    }
                }
                if flag {
                    self.dom_tree.get_mut(dom_label).unwrap().insert(*doms.0);
                }
            }
        }
        println!("dom tree is {:?}", &self.dom_tree)
    }

    pub fn get_pred(&mut self) {
        // let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (index, _) in self.function.blocks.iter().enumerate() {
            // block.successors
            let mut pred: HashSet<usize> = HashSet::new();
            for (i, b) in self.function.blocks.iter().enumerate() {
                if b.successors.contains(&index) {
                    // pred.push(i);
                    pred.insert(i);
                }
            }
            self.preds.insert(index, pred);
        }

        // return map;
    }
}
