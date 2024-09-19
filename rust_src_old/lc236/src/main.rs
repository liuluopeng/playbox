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
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let root = root.unwrap();
        let root_ref = root.borrow();

        let pp = p.clone();
        let qq = q.clone();

        if root == pp.unwrap() || root == qq.unwrap() {
            return Some(root.clone());
        }

        let left = Self::lowest_common_ancestor(root_ref.left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(root_ref.right.clone(), p.clone(), q.clone());

        if left.is_none() {
            return right;
        } else if right.is_none() {
            return left;
        } else {
            return Some(root.clone());
        }
    }
}

fn main() {
    // 创建节点
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));

    // 构建树结构
    node1.borrow_mut().left = Some(node2.clone());
    node1.borrow_mut().right = Some(node3.clone());

    // 输出树结构
    println!("树结构: {:?}", node1);
}
