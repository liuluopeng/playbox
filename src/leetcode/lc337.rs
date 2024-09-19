use crate::binary_tree::TreeNode;
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(7, Solution::rob(string2tree("[3,2,3,null,3,null,1]")));

        assert_eq!(9, Solution::rob(string2tree("[3,4,5,1,3,null,1]")));

        assert_eq!(7, Solution::rob(string2tree("[4,1,null,2,null,3]")));
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

struct Info {
    res: i32,

    pick_me_res: i32,
    do_not_pick_me_res: i32,
}
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find(root).res
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>) -> Info {
        let mut info = Info {
            res: 0,

            pick_me_res: 0,
            do_not_pick_me_res: 0,
        };

        if let Some(node) = root {
            let node = node.borrow();

            let left_info = Self::find(node.left.clone());
            let right_info = Self::find(node.right.clone());

            // 选我
            info.pick_me_res =
                node.val + left_info.do_not_pick_me_res + right_info.do_not_pick_me_res;

            info.do_not_pick_me_res = left_info.res + right_info.res;

            info.res = info.pick_me_res.max(info.do_not_pick_me_res);
        }

        info
    }
}
