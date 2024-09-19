use crate::binary_tree::TreeNode;
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            vec![2],
            Solution::tree_queries(
                string2tree("[1,3,4,2,null,6,5,null,null,null,null,null,7]"),
                vec![4]
            )
        );
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        let mut len = Self::get_size(root.clone());

        let mut dfn = vec![(0, 0, 0); len];
        let mut dfn_idx = 0;
        Self::find(root, 0, &mut dfn_idx, &mut dfn);

        // for (i, v) in dfn.iter().enumerate() {
        //     println!("{:?} : {:?}", i, v);
        // }

        let mut left_max_aux_arr = vec![dfn[0].2];
        let mut right_max_aux_arr = vec![dfn[len - 1].2];

        for i in 1..len {
            left_max_aux_arr.push(dfn[i].2.max(left_max_aux_arr[left_max_aux_arr.len() - 1]));
        }

        for i in (0..len - 1).rev() {
            right_max_aux_arr.push(dfn[i].2.max(right_max_aux_arr[right_max_aux_arr.len() - 1]));
        }
        right_max_aux_arr.reverse();

        let mut val_finder = vec![0; len]; // idx => dfn的数值

        for (idx, &ddd) in dfn.iter().enumerate() {
            val_finder[ddd.0 as usize - 1] = idx;
        }

        for &q in queries.iter() {
            // 去掉dfn范围内的结点. dfn_root ~ dfn_root + dfn_counter - 1.
            // 然后找到剩下的最大深度

            let target_idx = val_finder[q as usize - 1];

            let counter_of_q_tree = dfn[target_idx].1;

            let mut one_res = 0;

            if target_idx != 0 {}

            let mut left_side = 0;
            let mut right_side = 0;
            if target_idx > 0 {
                left_side = left_max_aux_arr[target_idx - 1];
            }
            if target_idx + counter_of_q_tree < len {
                right_side = right_max_aux_arr[target_idx + counter_of_q_tree];
            }

            one_res = left_side.max(right_side);

            res.push(one_res as i32);
        }

        res
    }

    pub fn find(
        root: Option<Rc<RefCell<TreeNode>>>,
        now_depth: usize,
        dfn_idx: &mut usize,
        dfn: &mut Vec<(i32, usize, usize)>,
    ) -> (usize, usize) {
        if let Some(node) = root {
            let node = node.borrow();
            let my_dfn_idx = *dfn_idx;

            *dfn_idx += 1;
            let mut counter = 0;
            let mut dist_to_root = 0;

            let left_res = Self::find(node.left.clone(), now_depth + 1, dfn_idx, dfn);
            let right_res = Self::find(node.right.clone(), now_depth + 1, dfn_idx, dfn);

            counter = left_res.0 + right_res.0 + 1;
            dist_to_root = now_depth;

            // dfn.push((node.val, counter, dist_to_root));

            dfn[my_dfn_idx] = (node.val, counter, dist_to_root);

            (counter, dist_to_root)
        } else {
            (0, 0)
        }
    }

    pub fn get_size(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(node) = root {
            let node = node.borrow();
            return 1 + Self::get_size(node.left.clone()) + Self::get_size(node.right.clone());
        } else {
            0
        }
    }
}
