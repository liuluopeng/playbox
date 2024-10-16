use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();

        for n in nums {
            m.insert(n, true);
        }

        let mut record = 0;
        // 寻找前一个数字存在不  不存在: 开始计数,一直往大了寻找   存在:跳过

        for (&k, &v) in m.iter() {
            if m.contains_key(&(k - 1)) {
                continue;
            } else {
                let mut cadi_record = 0;
                let mut curr_number = k;

                while m.contains_key(&curr_number) {
                    cadi_record += 1;
                    curr_number += 1;
                }

                record = cadi_record.max(record);
            }
        }

        record
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use crate::leetcode::leetcode_128::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }
}
