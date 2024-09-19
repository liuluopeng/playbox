use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = None;
        Solution::find(root, None, None, &mut res);

        if let Some(v) = res {
            v
        } else {
            0
        }
    }
    pub fn abs(a: i32) -> i32 {
        if a > 0 {
            return a;
        }
        -a
    }
    pub fn find(
        root: Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
        res: &mut Option<i32>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();

            let mut min = min;
            let mut max = max;

            // update max,min in this path:
            if min.is_none() {
                min = Some(node.val);
            } else {
                min = Some(min.unwrap().min(node.val));
            }
            if max.is_none() {
                max = Some(node.val);
            } else {
                max = Some(max.unwrap().max(node.val));
            }

            let left = node.left.as_ref();

            let left_node: Option<Rc<RefCell<TreeNode>>> = match left {
                None => None,
                Some(left) => Some(Rc::clone(left)),
            };

            let right = node.right.as_ref();
            let right_node = match right {
                None => None,
                Some(right) => Some(Rc::clone(right)),
            };

            if let Some(v) = *res {
                if v < Solution::abs(max.unwrap() - min.unwrap()) {
                    *res = Some(Solution::abs(max.unwrap() - min.unwrap()));
                }
            } else {
                *res = Some(Solution::abs(max.unwrap() - min.unwrap()));
            }

            Solution::find(left_node, min, max, res);
            Solution::find(right_node, min, max, res);
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
    use crate::{binary_tree::string2tree, solution::Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let st = "[1,2,3]";
        let st = "[8,3,10,1,6,null,14,null,null,4,7,13]";
        let st = "[]";
        let root = string2tree(st);

        assert_eq!(7, Solution::max_ancestor_diff(root));
    }
}
