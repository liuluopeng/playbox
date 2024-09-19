use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut area = 0;

        let mut stack = vec![];

        // 找 左侧第一个 较大 的
        for (i, &v) in height.iter().enumerate() {
            let mut one_turn_area = 0;
            while !stack.is_empty() && height[stack[stack.len() - 1]] <= v {
                let squirm_index = stack.pop().unwrap(); // 这个是最低的

                if stack.is_empty() {
                    break;
                }

                let left_board_index = stack[stack.len() - 1]; // 3 2 3  area=1
                let h_relav = height[i].min(height[left_board_index]) - height[squirm_index];

                area += (i - left_board_index + 1 - 2) as i32 * h_relav;
            }

            // if !stack.is_empty() {
            //     println!("{}:{}   {}", i, height[i], height[stack[stack.len() - 1]],);
            // } else {
            //     println!("{}:{} is biggest in its left", i, height[i]);
            // }

            // area += one_turn_area;

            stack.push(i);
        }

        area
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
        let root = string2tree(st);

        let arr = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        println!("{:?}", Solution::trap(arr));
    }
}
