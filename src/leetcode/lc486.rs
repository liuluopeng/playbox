impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();

        // A 从l到r能得到的最大结果.
        // A和B都想最大结果的情况下, A能得到的最大结果.

        let mut record = vec![vec![-1; nums.len()]; nums.len()];
        let res = Self::find(&nums, 0, nums.len() - 1, &mut record);

        res >= sum - res
    }

    pub fn find(nums: &Vec<i32>, l: usize, r: usize, record: &mut Vec<Vec<i32>>) -> i32 {
        if l == r {
            return nums[l];
        }

        if l + 1 == r {
            return nums[l].max(nums[r]);
        }

        if record[l][r] == -1 {
            let mut res = 0;

            // A选择l, 那么B面对[l + 1, r]选择之后可能是2种: [l + 2, r]  [l + 1, r - 1].

            let may1 = nums[l]
                + Self::find(nums, l + 2, r, record).min(Self::find(nums, l + 1, r - 1, record));

            // A:r   B:[l, r - 1] B选了之后: [l + 1, r - 1]   [l , r - 2]
            let may2 = nums[r]
                + Self::find(nums, l + 1, r - 1, record).min(Self::find(nums, l, r - 2, record));
            res = may1.max(may2);
            record[l][r] = res;
        }
        record[l][r]
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(false, Solution::predict_the_winner(vec![1, 5, 2]));
        assert_eq!(true, Solution::predict_the_winner(vec![1, 5, 233, 7]));

        assert_eq!(
            false,
            Solution::predict_the_winner(vec![0, 0, 7, 6, 5, 6, 1])
        );
        /*




        */

        assert_eq!(
            true,
            Solution::predict_the_winner(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
        );
    }
}
