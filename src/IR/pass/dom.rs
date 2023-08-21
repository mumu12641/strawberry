use std::collections::{HashMap, HashSet};

use crate::IR::abstract_present::AbstractFunction;

pub struct DOM {
    pub function: AbstractFunction,
}
impl DOM {
    pub fn get_dom(&mut self) {
        let mut dom: HashMap<usize, HashSet<usize>> = HashMap::new();
        // init every block -> all block
        let preds = self.get_pred();
        let size = self.function.blocks.len();
        for i in 0..size {
            if i == 0 {
                dom.insert(i, (0..1).collect());
            } else {
                dom.insert(i, (0..size).collect());
            }
        }
        let mut changed = true;
        while changed {
            changed = false;

            // for every block
            for index in 1..size {
                let mut pred_intersect: HashSet<usize> = HashSet::new();
                let pred = preds.get(&index).unwrap();
                // get its all pred
                for (i, k) in pred.iter().enumerate() {
                    if i == 0 {
                        pred_intersect = dom.get(k).unwrap().clone();
                    } else {
                        pred_intersect = &pred_intersect & &dom.get(k).unwrap();
                    }
                }
                let new_dom: HashSet<usize> = &pred_intersect | &HashSet::from_iter(vec![index]);

                if new_dom != dom.get(&index).unwrap().clone() {
                    changed = true;
                    dom.insert(index, new_dom);
                }
            }
            if !changed {
                break;
            }
        }
    }

    pub fn get_dom_frontier(&mut self) {}

    pub fn get_pred(&mut self) -> HashMap<usize, HashSet<usize>> {
        let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (index, _) in self.function.blocks.iter().enumerate() {
            // block.successors
            let mut pred: HashSet<usize> = HashSet::new();
            for (i, b) in self.function.blocks.iter().enumerate() {
                if b.successors.contains(&index) {
                    // pred.push(i);
                    pred.insert(i);
                }
            }
            map.insert(index, pred);
        }
        return map;
    }
}
