use std::{cell::RefCell, rc::Rc};

use crate::{binary_tree::TreeNode, linked_list::ListNode};

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums = vec![];
        let mut head = head.clone();
        let mut current_node = &mut head;
        while let Some(node) = current_node {
            nums.push(node.val);
            current_node = &mut current_node.as_mut().unwrap().next;
        }

        // println!("{:?}", nums);

        if nums.len() == 0 {
            return None;
        }

        Solution::build_tree(&nums, 0, nums.len() - 1)
    }

    pub fn build_tree(nums: &Vec<i32>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }

        let mid = (l + r) / 2;
        // 中点 构造 root
        let node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
        // println!("{:?} {:?} {:?}", l, r, node.borrow().val);
        if mid > 0 {
            node.borrow_mut().left = Solution::build_tree(nums, l, mid - 1);
        }
        node.borrow_mut().right = Solution::build_tree(nums, mid + 1, r);

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::{binary_tree::tree2string, linked_list::ListNode, solution::Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let arr = vec![-10, -3, 0, 5, 9];
        let arr = vec![];
        let head = ListNode::load_from_testcase(arr);
        // head.clone().unwrap().peek();

        let root = Solution::sorted_list_to_bst(head.clone());

        println!("{:?}", tree2string(root));
    }
}
