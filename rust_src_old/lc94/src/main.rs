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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;


struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        let mut res = vec![];

        Solution::_traversal(root,&mut res);
        res 
    }

    // 把遍历的结果记录到 规定的数组中
    pub fn _traversal(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {

        // Solution::_traversal(root, res)

        match root {
            Some(node) => {
                let node = node.borrow();
                Solution::_traversal(node.left.clone(), res);
                res.push(node.val);
                Solution::_traversal(node.right.clone(), res);
            }
            None => {}
        }
    }
}


fn main() {

}