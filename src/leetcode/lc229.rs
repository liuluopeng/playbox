use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        let mut freq = HashMap::new();

        for &v in nums.iter() {
            let f = freq.entry(v).or_insert(0);
            *f += 1;
        }

        for (k, v) in freq {
            // println!("{} {}", k, v);

            if v > nums.len() / 3 {
                res.push(k);
            }
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc229::Solution;

    #[test]
    fn it_works() {
        let arr = [1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1].to_vec();

        println!("{:?}", Solution::majority_element(arr));
    }
}
