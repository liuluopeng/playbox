use std::{cell::RefCell, rc::Rc};

use crate::{binary_tree::TreeNode, linked_list::ListNode};

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut size = 0;
        let mut current = head.as_ref();
        while let Some(node) = current {
            current = node.next.as_ref();
            size += 1;
        }

        if size <= 1 {
            return head;
        }

        let k = (k as usize) % size;

        if size <= 1 || k == 0 {
            return head;
        }

        let mut slow_node = Solution::get_slow_node(&mut head, k);

        println!("{:?} k th", slow_node);

        let mut tail_point = &mut slow_node;
        while let Some(node) = tail_point {
            println!("{:?} val tail point", node.val);
            if node.next.is_none() {
                break;
            }
            tail_point = &mut (*tail_point).as_mut().unwrap().next;
        }

        println!("{:?} tail node", tail_point);
        tail_point.as_mut().unwrap().next = head;

        // slow_node.as_mut().unwrap().next = None;

        slow_node
    }

    fn get_slow_node(head: &mut Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
        unsafe {
            let mut fast = head as *mut Option<Box<ListNode>>;

            let mut slow = head as *mut Option<Box<ListNode>>;

            let mut counter = 0;
            while counter < k {
                fast = &mut (*fast).as_mut().unwrap().next;
                counter += 1;
            }
            while (*fast).is_some() {
                fast = &mut (*fast).as_mut().unwrap().next;
                slow = &mut (*slow).as_mut().unwrap().next;
            }

            (*slow).take()
            // (*slow).clone()
        }
    }

    // pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    //     let k = k as usize;

    //     // let slow = None;

    //     let head = &mut head;

    //     let mut fast = head.as_mut();
    //     // let mut slow = head.as_mut(); // slow -> to_sp  ..... -> endk

    //     // let mut slow = fast; // slow -> to_sp  ..... -> endk
    //     // let mut slow = fast.as_mut().and_then(|node| node.next.as_mut());

    //     let mut slow = head.as_mut();

    //     // let mut pre_slow = head.as_mut();
    //     let mut counter = 0;
    //     while counter < k + 1 {
    //         if let Some(node) = fast {
    //             counter += 1;
    //             fast = node.next.as_mut();
    //         }
    //     }
    //     while let Some(node) = fast {
    //         fast = node.next.as_mut();
    //         // slow = slow.unwrap().next.as_mut();

    //         if let Some(slow_node) = slow {
    //             slow = slow_node.next.as_mut();
    //         }
    //     }

    //     println!("slow {:?} \n fast  {:?}", slow, fast);

    //     // 切断
    //     // let slow1 = slow.cloned();
    //     // let slow2 = slow.clone();

    //     // slow.as_mut().unwrap().next = None;

    //     // slow.as_mut().unwrap().val = 333;

    //     // Solution::modify_val(slow, 33333);

    //     head.as_ref().unwrap().peek();

    //     head.clone()
    // }
}
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
        let k = 3;
        let k = 2;
        // let k = 5;
        let head = Solution::rotate_right(head, k);
        head.unwrap().peek();
    }
}
