use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];
        for (i, &t) in temperatures.iter().enumerate().rev() {
            while !stack.is_empty() && t >= temperatures[stack[stack.len() - 1]] {
                stack.pop();
            }
            if stack.is_empty() {
                res.push(0);
            } else {
                res.push(stack[stack.len() - 1] as i32 - i as i32);
            }

            stack.push(i);
        }

        res.reverse();
        res
    }

    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];

        for (i, &h) in heights.iter().enumerate().rev() {
            let mut counter = 0;

            while !stack.is_empty() && h > heights[stack[stack.len() - 1]] {
                let basin_index = stack.pop().unwrap();
                counter += 1;
            }

            if !stack.is_empty() {
                counter += 1;
            }

            res.push(counter);

            stack.push(i);
        }

        res.reverse();

        res
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

        let heights = vec![10, 6, 8, 5, 11, 9];
        println!("{:?}", Solution::can_see_persons_count(heights));

        let tem = vec![73, 74, 75, 71, 69, 72, 76, 73];

        println!("{:?}", Solution::daily_temperatures(tem));
    }
}
