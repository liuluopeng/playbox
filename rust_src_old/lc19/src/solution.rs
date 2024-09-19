use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{binary_tree::TreeNode, linked_list::ListNode};

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));

        dummy_head.as_mut().unwrap().next = head.clone();

        let mut dest_node: Option<&Box<ListNode>> = head.as_ref();

        // let mut prev_node: Option<&Box<ListNode>> = dummy_head.as_ref();
        let mut prev_node = &mut dummy_head;

        let mut tail_point: Option<&Box<ListNode>> = head.as_ref();

        let mut counter = 0;
        while counter < n - 1 {
            if let Some(node) = tail_point {
                if node.next.is_some() {
                    tail_point = node.next.as_ref();

                    counter += 1;
                }
            }
        }

        // move tail_node to the tail of linked list
        while let Some(node) = tail_point {
            if node.next.is_some() {
                tail_point = node.next.as_ref();
                dest_node = dest_node.unwrap().next.as_ref();
                prev_node = &mut prev_node.as_mut().unwrap().next;
            } else {
                break;
            }
        }

        // println!(
        //     "prev:{:?}  \ndest:{:?} \ntail:{:?}",
        //     prev_node, dest_node, tail_point
        // );

        prev_node.as_mut().unwrap().next = prev_node.as_mut()?.next.as_mut()?.next.clone();

        dummy_head.unwrap().next
    }
}
// write my solution here:

/// 占位 用来引入需要的函数
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

        let data = vec![1, 2, 3, 4, 5];
        let head = ListNode::load_from_testcase(data);

        let k = 2;

        let head = Solution::remove_nth_from_end(head, k);

        head.unwrap().peek();
    }
}
