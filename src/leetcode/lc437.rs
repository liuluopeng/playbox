use crate::binary_tree::TreeNode;
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::path_sum(string2tree("[10,5,-3,3,2,null,11,3,-2,null,1]"), 8)
        );

        assert_eq!(
            3,
            Solution::path_sum(string2tree("[5,4,8,11,null,13,4,7,2,null,null,5,1]"), 22)
        );

        assert_eq!(0, Solution::path_sum(string2tree("[1000000000,1000000000,null,294967296,null,1000000000,null,1000000000,null,1000000000]"),0));
    }
}
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
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let target_sum = target_sum as i64;

        Self::find(root, target_sum) as i32
    }

    // 用root作为路径的开始, 的 , 答案.   root必须是路径的第一个.
    fn find(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> usize {
        if let Some(node) = root.clone() {
            let me_res = Self::f2(Some(node.clone()), target_sum);

            let node = node.borrow();

            return me_res
                + Self::find(node.left.clone(), target_sum)
                + Self::find(node.right.clone(), target_sum);
        } else {
            return 0;
        }
    }

    fn f2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> usize {
        if let Some(node) = root {
            let node = node.borrow();

            let mut res = 0;

            if target_sum - node.val as i64 == 0 {
                // 遇到一个路径.
                res = 1;
            }

            let l_res = Self::f2(node.left.clone(), target_sum - node.val as i64);
            let r_res = Self::f2(node.right.clone(), target_sum - node.val as i64);

            res + l_res + r_res
        } else {
            return 0;
        }
    }
}
