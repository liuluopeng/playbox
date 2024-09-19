use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{binary_tree::TreeNode, linked_list::ListNode};
use rand::prelude::SliceRandom;

pub struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode::new(-1)));
    let mut p = &mut dummy_head;
    let mut head = head;
    let mut queue = vec![];
    while let Some(mut node) = head {
        head = node.next.take();
        queue.push(node);
    }
    println!("{}",queue.len());
    queue.sort_by(|a, b| b.val.cmp(&a.val));


    while queue.len() > 18000 {
        let node = queue.pop().unwrap();
        p.as_mut().unwrap().next = Some(node);
        p = &mut p.as_mut().unwrap().next;

    }

    dummy_head.unwrap().next
    }
}
pub struct Solution2;
impl Solution2 {
    pub fn ppp(root: Option<Rc<RefCell<TreeNode>>>) {}
}

#[cfg(test)]
mod tests {

    use crate::{binary_tree::string2tree, linked_list::ListNode, solution::Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let st = "[1,2,3]";
        let root = string2tree(st);

        let arr = vec![-1, 5, 3, 4, 0];
    }
}
