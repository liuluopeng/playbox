use crate::binary_tree::TreeNode;
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            20,
            Solution::max_sum_bst(string2tree(
                "[1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]"
            ))
        );

        assert_eq!(2, Solution::max_sum_bst(string2tree("[4,3,null,1,2]")));

        assert_eq!(0, Solution::max_sum_bst(string2tree("[-4,-2,-5]")));

        assert_eq!(
            14,
            Solution::max_sum_bst(string2tree(
                "[4,8,null,6,1,9,null,-5,4,null,null,null,-3,null,10]"
            ))
        );
    }
}




use std::cell::RefCell;
use std::rc::Rc;

struct Info {
    is_bst: bool,

    global_max_sum: i32, // 题意.   我不是bst的时候, 我仅传递结果给父亲
    curr_my_sum: i32,    // 我是bst的时候保存的sum.

    min_of_my_bst: i32,
    max_of_my_bst: i32,
}

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find(root).global_max_sum
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>) -> Info {
        let mut info = Info {
            is_bst: false,
            global_max_sum: 0,
            curr_my_sum: 0,
            // global_max_sum: 0,
            min_of_my_bst: 0,
            max_of_my_bst: 0,
        };

        if let Some(node) = root {
            let node = node.borrow();
            let left_res = Self::find(node.left.clone());
            let right_res = Self::find(node.right.clone());

            if left_res.is_bst
                && right_res.is_bst
                && left_res.max_of_my_bst < node.val
                && right_res.min_of_my_bst > node.val
            {
                // 可以构成更大的bst

                info.is_bst = true;

                if node.left.is_none() {
                    info.min_of_my_bst = node.val;
                } else {
                    info.min_of_my_bst = left_res.min_of_my_bst;
                }
                if node.right.is_none() {
                    info.max_of_my_bst = node.val;
                } else {
                    info.max_of_my_bst = right_res.max_of_my_bst;
                }

                info.curr_my_sum = left_res.curr_my_sum + right_res.curr_my_sum + node.val;
                info.global_max_sum = left_res.global_max_sum.max(right_res.global_max_sum);

                info.global_max_sum = info.curr_my_sum.max(info.global_max_sum);
            } else {
                info.is_bst = false;

                // info.curr_my_sum = left_res.curr_my_sum.max(right_res.curr_my_sum);
                info.global_max_sum = left_res.global_max_sum.max(right_res.global_max_sum);
            }
        } else {
            info.is_bst = true;
            info.global_max_sum = 0;
            info.curr_my_sum = 0;
            info.min_of_my_bst = i32::MAX;
            info.max_of_my_bst = i32::MIN;
        }

        info
    }
}
