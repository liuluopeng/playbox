impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.insert(0, 1);
        nums.push(1);

        let mut record = vec![vec![-1; nums.len()]; nums.len()];

        Self::find(&nums, 1, nums.len() - 2, &mut record)
    }

    // start ~ end 范围内, 最后爆的气球
    pub fn find(
        nums: &Vec<i32>,
        start_idx: usize,
        end_idx: usize,
        record: &mut Vec<Vec<i32>>,
    ) -> i32 {
        // println!("{}{}", start_idx, end_idx);

        if start_idx == 0 {
            return 0;
        }
        if end_idx == nums.len() - 1 {
            return 0;
        }

        if start_idx == end_idx {
            return nums[start_idx] * nums[start_idx - 1] * nums[start_idx + 1];
        }

        if record[start_idx][end_idx] == -1 {
            let mut res = 0;

            for k_idx in start_idx..=end_idx {
                // println!("{}", k_idx);

                let one_res = nums[start_idx - 1] * nums[k_idx] * nums[end_idx + 1]
                    + Self::find(nums, start_idx, k_idx - 1, record)
                    + Self::find(nums, k_idx + 1, end_idx, record);

                // println!(
                //     "{}  * {}  * {}",
                //     nums[start_idx - 1],
                //     nums[k_idx],
                //     nums[end_idx + 1]
                // );

                res = res.max(one_res);
            }

            if res == 0 {
                // println!("nothing {} {}", start_idx, end_idx);
            }

            record[start_idx][end_idx] = res;
        }

        record[start_idx][end_idx]
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(167, Solution::max_coins(vec![3, 1, 5, 8]));

        assert_eq!(
            3630,
            Solution::max_coins(vec![
                8, 2, 6, 8, 9, 8, 1, 4, 1, 5, 3, 0, 7, 7, 0, 4, 2, 2, 5
            ])
        );
    }
}
