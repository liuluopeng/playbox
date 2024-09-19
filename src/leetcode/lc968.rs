use crate::binary_tree::TreeNode;
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::min_camera_cover(string2tree("[0,0,null,0,0]")));

        assert_eq!(
            2,
            Solution::min_camera_cover(string2tree("[0,0,null,0,null,0,null,null,0]"))
        );
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
    node_counter: i32,
    be_watched: bool,
    setted_camera: bool,
}
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let info = Self::find(root);

        let mut res_without_root = info.res;

        if info.be_watched == false && info.setted_camera == false {
            res_without_root += 1;
        }

        res_without_root
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>) -> Info {
        let mut info = Info {
            res: 0,
            be_watched: true,
            setted_camera: false,
            node_counter: 0,
        };

        if let Some(node) = root {
            let node = node.borrow();

            let left_res = Self::find(node.left.clone());
            let right_res = Self::find(node.right.clone());

            info.node_counter = left_res.node_counter + right_res.node_counter + 1;

            if (left_res.be_watched == false && left_res.setted_camera == false)
                || (right_res.be_watched == false && right_res.setted_camera == false)
            {
                info.setted_camera = true;

                info.res = left_res.res + right_res.res + 1;
                info.be_watched = true;
            } else if (left_res.be_watched && left_res.setted_camera == false)
                && (right_res.be_watched && right_res.setted_camera == false)
            {
                info.setted_camera = false;
                info.be_watched = false;
                info.res = left_res.res + right_res.res;
            } else {
                info.setted_camera = false;
                info.be_watched = true;
                info.res = left_res.res + right_res.res;
            }
        } else {
        }

        info
    }
}
