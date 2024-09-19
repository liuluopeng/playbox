use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = -2147483648;
        Solution::find(root, &mut res);
        res
    }

    pub fn find(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();

            let left = node.left.as_ref().map(Rc::clone);
            let right = node.right.as_ref().map(Rc::clone);

            let mut left_sum = Solution::find(left, res);
            if left_sum < 0 {
                left_sum = 0;
            }
            let mut right_sum = Solution::find(right, res);
            if right_sum < 0 {
                right_sum = 0;
            }
            // println!("node:{}  left:{} right:{}", node.val, left_sum, right_sum);

            let sum = node.val + left_sum + right_sum;

            if sum > *res {
                *res = sum;
            }

            return node.val + left_sum.max(right_sum);
        } else {
            return 0;
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
        let st = "[-10,9,20,null,null,15,7]";

        let st = "[2,-1]";
        let root = string2tree(st);

        let res = Solution::max_path_sum(root);
        println!("res: {:?}", res);
    }
}
