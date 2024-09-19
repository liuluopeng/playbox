pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut memo = vec![1; nums.len()]; // memo[i]: 以nums[i]结尾的子序列长度的最长可能

        for i in 0..nums.len() {
            //

            for j in 0..i {
                if nums[j] < nums[i] {
                    memo[i] = memo[i].max(1 + memo[j]);
                }
            }
        }

        for i in memo {
            res = res.max(i);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }
}
