// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut res = 0;
        Self::find(root, &mut res, low, high);
        res
    }

    pub fn find(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32, low: i32, high: i32) {
        match root {
            None => {}
            Some(node) => {
                let node = node.borrow();

                Self::find(node.left.clone(), res, low, high);
                if node.val > low {
                    *res += node.val;
                }

                if node.val > high {
                    return;
                }
                Self::find(node.right.clone(), res, low, high);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
