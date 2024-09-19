impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        Self::find(&nums, nums.len() - 1, target)
    }

    fn find(nums: &Vec<i32>, idx: usize, target: i32) -> i32 {
        if idx == 0 {
            let mut res = 0;
            if nums[0] == target {
                res += 1;
            }
            if -nums[0] == target {
                res += 1;
            } else {
            }
            return res;
        }

        //  target - nums[idx] + nums[idx] = target
        let case1 = Self::find(nums, idx - 1, target - nums[idx]);
        // target + nums[idx] - nums[idx] = target
        let case2 = Self::find(nums, idx - 1, target + nums[idx]);

        case1 + case2
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    }
}
