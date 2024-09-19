impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut record = vec![-1; nums.len()];

        Self::find(&mut record, &nums, nums.len() - 1);

        let mut res = 0;
        println!("{:?}", record);

        for n in record {
            res = res.max(n);
        }

        res
    }

    fn find(record: &mut Vec<i32>, nums: &Vec<i32>, idx: usize) -> i32 {
        // find(idx): 代表一个临时答案,这个答案必须包含nums[idx],以及idx的左侧.
        if idx == 0 {
            record[0] = nums[0];
            return record[0];
        }
        if idx == 1 {
            record[1] = nums[1];
            return record[1];
        }

        if record[idx] == -1 {
            // idx - 1 不考虑.

            // 考虑 dp[idx - 2]  dp[idx - 3]

            let mut maybe2 = 0;

            Self::find(record, nums, idx - 1);

            let maybe1 = Self::find(record, nums, idx - 2);

            if idx >= 3 {
                maybe2 = Self::find(record, nums, idx - 3);
            }

            // println!("idx: {} maybe1: {} maybe2: {}", idx, maybe1, maybe2);

            record[idx] = maybe1.max(maybe2) + nums[idx];
        }

        record[idx]
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        let nums = vec![1, 3, 1, 3, 100];
        assert_eq!(103, Solution::rob(nums));

        let nums = vec![1, 2, 3, 1];
        assert_eq!(4, Solution::rob(nums));

        let nums = vec![2, 1];
        assert_eq!(2, Solution::rob(nums));
    }
}
