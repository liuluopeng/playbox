use std::{cell::RefCell, rc::Rc};

use crate::{binary_tree::TreeNode, solution::Solution};

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::largest_bst_subtree(string2tree("[10,5,15,1,8,null,7]"))
        );

        assert_eq!(
            2,
            Solution::largest_bst_subtree(string2tree(
                "[4,2,7,2,3,5,null,2,null,null,null,null,null,1]"
            ))
        )
    }
}

#[derive(Debug)]
struct Info {
    min_value_of_my_tree: i32,
    max_value_of_my_tree: i32,
    am_i_bst: bool,
    result_of_me: i32,
}

impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find(root).result_of_me
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>) -> Info {
        let mut info = Info {
            min_value_of_my_tree: 0,
            max_value_of_my_tree: 0,
            am_i_bst: false,
            result_of_me: 0,
        };

        if let Some(node) = root {
            let node = node.borrow();

            let my_left_info = Self::find(node.left.clone());
            let my_right_info = Self::find(node.right.clone());

            // 两侧都是bst
            if my_left_info.am_i_bst
                && my_right_info.am_i_bst
                && my_left_info.max_value_of_my_tree < node.val
                && node.val < my_right_info.min_value_of_my_tree
            {
                // 看看自己是否处于中间位置, 从而构成更大的bst

                info.am_i_bst = true;
                info.result_of_me = my_left_info.result_of_me + my_right_info.result_of_me + 1;

                if node.left.is_none() {
                    info.min_value_of_my_tree = node.val;
                } else {
                    info.min_value_of_my_tree = my_left_info.min_value_of_my_tree;
                }
                if node.right.is_none() {
                    // 右侧是空的 => root自己是最大的.
                    info.max_value_of_my_tree = node.val;
                } else {
                    info.max_value_of_my_tree = my_right_info.max_value_of_my_tree;
                }
            } else {
                // 不用看了, 自己不是bst, 自己的结果是两侧中较大的.
                info.am_i_bst = false;
                info.result_of_me = my_left_info.result_of_me.max(my_right_info.result_of_me);
            }

            println!(" val: {} \n info {:?}", node.val, info);
        } else {
            // root is null
            info.am_i_bst = true;
            info.result_of_me = 0;
            // 空结点 需要  使得  叶子的最大值最小值都是叶子自己的value
            info.min_value_of_my_tree = i32::MAX;
            info.max_value_of_my_tree = i32::MIN;
        }

        info
    }
}
