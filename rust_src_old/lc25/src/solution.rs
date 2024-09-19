use crate::binary_tree::TreeNode;
use crate::linked_list::ListNode;

pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;

        res
    }

    pub fn find(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();

            let left = node.left.as_ref().map(Rc::clone);
            let right = node.right.as_ref().map(Rc::clone);

            let left_sum = Solution::find(left, res);
            let right_sum = Solution::find(right, res);

            let sum = node.val + left_sum + right_sum;

            if sum > *res {
                *res = sum;
            }
            return sum;
        } else {
            return 0;
        }
    }
}
// write my solution here:
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut res = vec![];

        while current.is_some() {
            let mut slice = vec![];
            for i in 0..k {
                if let Some(node) = current.clone() {
                    slice.push(current.clone());
                    current = node.next;
                }
            }
            res.push(slice);
        }

        for i in 0..res.len() {
            if res[i].len() == k as usize {
                for j in 0..res[i].len() {
                    if let Some(mut node) = res[i][j].as_mut() {
                        node.next = None;
                        println!("{:?} val", node.val);
                    }
                }

                res[i].reverse();
            } else {
            }
        }

        for i in 0..res.len() {
            println!("{}", i);
            for j in 0..res[i].len() {
                println!("{:?}", res[i][j]);
            }
        }

        let mut res1d: Vec<Option<Box<ListNode>>> = vec![];
        for i in res {
            for j in i {
                res1d.push(j);
            }
        }

        let mut dummy_head = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy_head.next;

        // for i in 0..res1d.len() - 1 {

        //     let next_node: Option<Box<ListNode>> = res1d[i + 1].take();

        //     // let next_node: Option<Box<ListNode>> = res1d[i + 1].clone();

        //     // let next_node = & res1d[i + 1];

        //     if let Some(ref mut node) = res1d[i] {
        //         // println!("{:?} {:?}", node.val, next_node.clone().unwrap().val);
        //         node.next = next_node;

        //         node.peek();
        //     }
        // }

        // let a = res1d.pop();

        for i in 0..res1d.len() {
            // *tail = res1d.pop().unwrap();

            *tail = res1d.remove(0);
            tail = &mut tail.as_mut().unwrap().next;
        }

        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::{linked_list::ListNode, solution::Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let arr = vec![1, 2, 3, 4, 5];
        let head = ListNode::load_from_testcase(arr);
        let k = 3;

        let head = Solution::reverse_k_group(head, k);
        if let Some(node) = head {
            println!("结果是:");
            node.peek();
        }
    }
}
