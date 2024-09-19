impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut res = vec![];

        let nums: Vec<i32> = num
            .bytes()
            .into_iter()
            .map(|b| (b - '0' as u8) as i32)
            .collect();

        Self::find(&mut vec![], &nums, 0, &mut res, &mut false);

        // println!("final result: {:?}", res);
        res
    }

    pub fn find(
        now: &mut Vec<i32>,
        nums: &Vec<i32>,
        len_start: usize, // 继续寻找的开始位置
        first_meet_res: &mut Vec<i32>,
        get_a_answer: &mut bool,
    ) {
        if *get_a_answer {
            return;
        }

        if now.len() >= 3 {
            let ll = now.len();

            if now[ll - 2] as i64 + now[ll - 3] as i64 > i32::MAX.into() {
                return;
            }

            if now[ll - 1] != now[ll - 2] + now[ll - 3] {
                return;
            }
        }

        if now.len() >= 3 && len_start == nums.len() {
            // println!("包含所有数字的now: {:?}", now);
            *first_meet_res = now.to_vec();
            *get_a_answer = true;
            return;
        }

        for k in (len_start)..nums.len() {
            // println!("k: {}  now: {:?}", k, now);

            // 创建一个片段.包含nums[start..k]
            let mut new_num: i64 = 0;

            for digit in len_start..=k {
                // 前导0
                if digit == len_start && nums[digit] == 0 {
                    break;
                }

                new_num = Self::add_lit(new_num, nums[digit]);
                if new_num > i32::MAX.into() {
                    return;
                }
            }

            if new_num == 0 && nums[len_start] == 0 && k > len_start {
                continue;
            }

            now.push(new_num as i32);
            Self::find(now, nums, k + 1, first_meet_res, get_a_answer);
            now.pop();
        }
    }

    pub fn add_lit(num: i64, new_digit: i32) -> i64 {
        let mut num = num;
        (num as i64 * 10 + new_digit as i64) as i64
    }
}

use crate::solution::Solution;
#[cfg(test)]
mod tests {
    use crate::{
        solution::{self, Solution},
        util::vec_2d_leetcode,
    };

    #[test]
    fn it_works() {
        println!(
            "{:?}",
            Solution::split_into_fibonacci("41006539810000000009999999990".to_string())
        );

        println!("{:?}", Solution::split_into_fibonacci("0123".to_string()));

        println!("{:?}", Solution::split_into_fibonacci("020406".to_string()));

        println!(
            "{:?}",
            Solution::split_into_fibonacci("1101111".to_string())
        );
    }
}
