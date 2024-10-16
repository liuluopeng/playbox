use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        // let mut record = HashMap::new();

        let mut sum_list = vec![0];

        for i in 0..nums.len() {
            sum_list.push(sum_list[sum_list.len() - 1] + nums[i]);
        }

        let mut cnt = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let sum = sum_list[j + 1] - sum_list[i];
                // record.insert((i, j), sum);
                if sum == k {
                    cnt += 1;
                }
            }
        }

        cnt
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use crate::leetcode::leetcode_560::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            5,
            Solution::subarray_sum(
                [1, 2, 3, 4, 5, 6, 7, 1, 23, 21, 3, 1, 2, 1, 1, 1, 1, 1, 12, 2, 3, 2, 3, 2, 2]
                    .to_vec(),
                22
            )
        );

        assert_eq!(2, Solution::subarray_sum([1, 1, 1].to_vec(), 2));
    }
}
