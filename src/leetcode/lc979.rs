// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use crate::binary_tree::TreeNode;
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::distribute_coins(string2tree("[3,0,0]")));
        assert_eq!(3, Solution::distribute_coins(string2tree("[0,3,0]")));
    }
}

struct Info {
    res_mine_tree: i32,
    // step_mine_tree: i32,
    value_sum: i32,
    node_counter: i32, // 结点个数
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find(root).res_mine_tree
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>) -> Info {
        let mut info = Info {
            res_mine_tree: 0,
            value_sum: 0,
            node_counter: 0,
        };
        if let Some(node) = root {
            let node = node.borrow();

            let left_res = Self::find(node.left.clone());
            let right_res = Self::find(node.right.clone());

            info.node_counter = left_res.node_counter + right_res.node_counter + 1;
            info.value_sum = left_res.value_sum + right_res.value_sum + node.val;

            info.res_mine_tree = (left_res.value_sum - left_res.node_counter).abs()
                + (right_res.value_sum - right_res.node_counter).abs()
                + left_res.res_mine_tree
                + right_res.res_mine_tree;
        } else {
        }

        info
    }
}
