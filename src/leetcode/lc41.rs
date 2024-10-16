use std::{collections::HashMap, i32};

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut record = HashMap::new();

        for n in nums {
            if n > 0 {
                record.insert(n, true);
            }
        }

        let mut res = 0;
        for i in 1..=i32::MAX {
            if !record.contains_key(&i) {
                res = i;
                break;
            }
        }
        res
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
