use std::{cell::RefCell, rc::Rc};

use crate::{binary_tree::TreeNode, linked_list::ListNode};

pub struct Solution;

// write my solution here:

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // collect all paths into a vector
        let mut paths = vec![];

        Self::find_path(root, &mut paths, &mut vec![]);

        let mut sub_vec = vec![];
        let mut current_node = head.as_ref();
        while let Some(node) = current_node {
            sub_vec.push(node.val);
            current_node = node.next.as_ref();
        }

        for p in paths {
            // println!("{:?}   {:?}", p, sub_vec);

            let res = Self::KMP(&p, &sub_vec);
            if res {
                return true;
            }
        }

        false
    }

    pub fn KMP(big_vec: &Vec<i32>, sub_vec: &Vec<i32>) -> bool {
        let next = Self::gen_next(sub_vec);

        let mut idx_big = 0;

        let mut idx_small = 0;

        while idx_big < big_vec.len() && idx_small < sub_vec.len() {
            if big_vec[idx_big] == sub_vec[idx_small] {
                idx_big += 1;
                idx_small += 1;
            } else {
                //  'a' != 'b'
                if idx_small == 0 {
                    // 辨析 next[idx_small]==0   idx_small==0
                    // 放弃, 往后移动一个, 重新开始匹配
                    idx_big += 1;
                } else {
                    idx_small = next[idx_small] as usize;
                }
            }
        }

        // println!("idx small:  {:?}", idx_small);

        if idx_small == sub_vec.len() {
            true
        } else {
            false
        }
    }

    pub fn gen_next(sub_vec: &Vec<i32>) -> Vec<i32> {
        let mut next = vec![-1; sub_vec.len()];

        for idx in 1..sub_vec.len() {
            // println!("next: {:?}", next);

            let mut flex_ctr = idx - 1;

            while next[flex_ctr] >= 0 && sub_vec[next[flex_ctr] as usize] != sub_vec[idx - 1] {
                flex_ctr = next[flex_ctr] as usize;
            }

            next[idx] = next[flex_ctr] + 1;
        }

        // println!("final next: {:?}", next);
        next
    }

    pub fn find_path(
        root: Option<Rc<RefCell<TreeNode>>>,
        paths: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            path.push(node.val);

            if node.left.is_none() && node.right.is_none() {
                // leaf node
                paths.push(path.to_vec());
                // path.clear();
            }

            Self::find_path(node.left.clone(), paths, &mut path.clone());

            Self::find_path(node.right.clone(), paths, &mut path.clone());
        } else {
            // paths.push(path.to_vec());
        }
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

        let st = "[1,2,3]";
        let root = string2tree(st);

        let head = ListNode::load_from_testcase(vec![4, 2, 8]);
        let root = string2tree("[1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]");

        println!("{:?}", Solution::is_sub_path(head, root));
    }
}
