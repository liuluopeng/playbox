impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::find_01(&mut res, &mut vec![], 0, &nums);
        res
    }

    // 两种解空间:
    // https://www.cnblogs.com/waring/p/4551218.html

    fn find_01(res: &mut Vec<Vec<i32>>, now: &mut Vec<i32>, idx: usize, nums: &Vec<i32>) {
        if idx >= nums.len() {
            res.push(now.to_vec());
            return;
        }

        // 选择当前的元素
        now.push(nums[idx]);
        Self::find_01(res, now, idx + 1, nums);
        now.pop();

        // 不选择当前元素
        Self::find_01(res, now, idx + 1, nums);
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::old_vec_2d_leetcode};

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3];
        println!("{:?}", Solution::subsets(nums));
    }
}
