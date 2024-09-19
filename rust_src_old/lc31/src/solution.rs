use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut ordered = nums.clone();
        ordered.sort();

        // println!("ordered: {:?}", ordered);

        let mut rev = ordered.clone();
        rev.reverse();

        if rev == nums.to_vec() {
            *nums = ordered;
            return;
        }

        let mut last_inc_i = nums.len() - 2;
        let mut last_inc_j = nums.len() - 1;
        for idx in (1..nums.len()).rev() {
            if nums[idx - 1] < nums[idx] {
                last_inc_i = idx - 1;
                last_inc_j = idx;
                break;
            }
        }

        // println!("{} {}   {:?} noth", last_inc_i, last_inc_j, nums);

        let mut k = ordered.len() - 1;
        for idx in (last_inc_i..nums.len()).rev() {
            if nums[idx] > nums[last_inc_i] {
                k = idx;
                break;
            }
        }
        nums.swap(last_inc_i, k);
        // println!("after swap k {:?}", nums);

        let mut counter = 0;
        for idx in last_inc_j..nums.len() {
            let swap1 = last_inc_j + counter;
            let swap2 = nums.len() - 1 - counter;

            if swap1 < swap2 {
                nums.swap(swap1, swap2);
                counter += 1;
            } else {
                break;
            }
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

        let mut arr = vec![1, 2, 3, 6, 4, 5];
        // let mut arr = vec![1, 3, 2];
        Solution::next_permutation(&mut arr);
        println!("{:?}", arr);
    }
}
