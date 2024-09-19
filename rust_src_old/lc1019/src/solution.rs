use std::{cell::RefCell, rc::Rc};

use crate::{binary_tree::TreeNode, linked_list::ListNode};

pub struct Solution;

// write my solution here:
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut arr = vec![];

        let mut current_node = head.as_ref();

        let mut data = vec![];

        while let Some(node) = current_node {
            data.push(node.val);

            current_node = node.next.as_ref();
        }

        let mut stack = vec![];

        // 739. 每日温度 - 力扣（LeetCode） https://leetcode.cn/problems/daily-temperatures/solutions/2470179/shi-pin-jiang-qing-chu-wei-shi-yao-yao-y-k0ks/
        for (i, &v) in data.iter().enumerate().rev() {
            while !stack.is_empty() && v >= data[stack[stack.len() - 1]] {
                stack.pop();
            }

            if stack.is_empty() {
                arr.push(0);
            } else {
                arr.push(data[stack[stack.len() - 1]]);
            }

            stack.push(i);
        }

        arr.reverse();
        arr 
    }
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


        let arr = vec![2, 7, 4, 3, 5];
        let head = ListNode::load_from_testcase(arr);
        let arr = Solution::next_larger_nodes(head);
        println!("{:?}", arr);
    }
}
