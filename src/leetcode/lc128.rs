impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut record = HashMap::new();

        let mut counter = 1;

        for &v in nums.iter() {
            record.insert(v, true);
        }

        // for &v in nums.iter() {
        //     if record.contains_key(&(v - 1)) {
        //         counter += 1;
        //     }
        // }

        for (k, v) in &record {
            if record.contains_key(&(k - 1)) {
                counter += 1;
            }
        }
        counter
    }
}

use std::collections::HashMap;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]))
    }
}
