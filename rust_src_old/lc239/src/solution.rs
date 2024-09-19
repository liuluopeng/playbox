use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let k = k as usize;
        let mut dequeue = vec![]; // 一个双端队列

        for (i, v) in nums.iter().enumerate() {
            // 右边出去的: 是我提前判断的
            // 左边出去的: 是时间流动的结果.

            while !dequeue.is_empty() && nums[dequeue[dequeue.len() - 1]] < *v {
                dequeue.pop();
            }

            dequeue.push(i);

            if i - dequeue[0] > k - 1 {
                dequeue.remove(0);
            }

            if i >= k - 1 {
                res.push(nums[dequeue[0]]);
            }
        }

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
        // assert_eq!(2 + 2, 43);

        let arr = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        println!("res: {:?}", Solution::max_sliding_window(arr, k));
    }
}
