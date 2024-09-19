impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.clone();

        nums.sort_by(|a, b| {
            let mut a_b = a.to_string();
            a_b.push_str(&b.to_string());

            let mut b_a = b.to_string();
            b_a.push_str(&a.to_string());

            // a_b.partial_cmp(&b_a).unwrap()
            b_a.partial_cmp(&a_b).unwrap()
        });

        let res = nums
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("");

        if nums[0] == 0 { // 特例是一个无效的数字
            return String::from("0");
        }

        res
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            String::from("9534330"),
            Solution::largest_number(vec![3, 30, 34, 5, 9])
        );
    }
}
