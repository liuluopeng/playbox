pub struct Solution;

pub struct Solution2;

// write my solution here:
use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let root = root.clone();
        let mut inorder_res = vec![];

        let mut modi1 = &mut None;
        let mut modi2 = &mut None;
        Solution::inorder(root, &mut inorder_res, &mut modi1, &mut modi2);

        // for i in inorder_res {
        //     println!(" {:?}", i);
        // }

        // for i in 0..inorder_res.len() {

        //     let prev
        // }

        println!("modi1 {:?} \n modi2 {:?}", modi1, modi2);

        if let (Some(mut node1), Some(mut node2)) = (modi1.clone(), modi2.clone()) {
            let mut node1 = node1.borrow_mut();
            let mut node2 = node2.borrow_mut();

            let tmp = node1.val;
            node1.val = node2.val;
            node2.val = tmp;
        }
    }

    pub fn inorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        inorder_res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,

        modi1: &mut Option<Rc<RefCell<TreeNode>>>,
        modi2: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(node) = root.clone() {
            let mut node = node.borrow();

            let left_node = node.left.clone();
            let right_node = node.right.clone();

            Solution::inorder(left_node, inorder_res, modi1, modi2);

            println!("{:?} curr", node.val);

            if inorder_res.len() > 0 {
                // let last = inorder_res.pop();
                let last: Option<Rc<RefCell<TreeNode>>> =
                    inorder_res[inorder_res.len() - 1].clone();

                if let Some(last_node) = last.clone() {
                    let mut last_node = last_node.borrow();

                    // println!("last: {} curr: {}  ", last_node.val, node.val);

                    if last_node.val > node.val {
                        if *modi1 == None {
                            *modi1 = last.clone();
                        }

                        *modi2 = root.clone();
                    }
                } else {
                    println!("no last . curr: {}", node.val);
                }
            }

            inorder_res.push(root);

            Solution::inorder(right_node, inorder_res, modi1, modi2);
        }
    }
}

impl Solution2 {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::inorder_traversal(root, &mut None);
    }

    pub fn inorder_traversal(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(current) = node {
            // Traverse the left subtree
            Solution2::inorder_traversal(&mut current.borrow().left.clone(), prev);

            // Current node processing
            if let Some(prev_node) = prev {
                let mut current_borrow = current.borrow_mut();
                let mut prev_borrow = prev_node.borrow_mut();
                if current_borrow.val < prev_borrow.val {
                    let tmp = current_borrow.val;

                    current_borrow.val = prev_borrow.val;
                    prev_borrow.val = tmp;
                }
            }

            // Update the previous node to be the current node
            *prev = Some(current.clone());

            // Traverse the right subtree
            Solution2::inorder_traversal(&mut current.borrow().right.clone(), prev);
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        binary_tree::{string2tree, tree2string},
        solution::Solution,
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let st = "[3,1,4,null,null,2]";
        let st = "[1,3,null,null,2]";
        let mut root = string2tree(st);
        Solution::recover_tree(&mut root);
        println!("{:}", tree2string(root));
    }
}
