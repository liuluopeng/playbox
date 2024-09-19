pub struct Solution;

use std::collections::HashMap;

impl Solution {
    /// 乘法竖式会不会出现进位超过99的情况:
    /// 在乘法竖式中，进位最多只会影响到相邻的两位数字。因此，无论是在十进制还是其他进制下，进位不会使得相邻两位数字的和超过当前进制数减去1。

    pub fn multiply(num1: String, num2: String) -> String {
        let size = 1000;
        let mut dig_list = vec![0; size];

        let my_dict: HashMap<char, i32> = HashMap::from([
            ('1', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('0', 0),
        ]);

        let mut chars1: Vec<char> = num1.chars().collect();
        let mut chars2: Vec<char> = num2.chars().collect();

        for i in (0..chars2.len()).rev() {
            for j in (0..chars1.len()).rev() {
                let i_sm_end = chars2.len() - 1 - i;
                let j_sm_end = chars1.len() - 1 - j;

                let dig_mul_dig =
                    my_dict.get(&chars2[i]).unwrap() * my_dict.get(&chars1[j]).unwrap();

                let base_index_sm_end = i_sm_end + j_sm_end;

                dig_list[size - 1 - base_index_sm_end] += dig_mul_dig;

                let mut watch_index = size - 1 - base_index_sm_end;

                while dig_list[watch_index] > 9 {
                    // 一直往前, 直到摊平.
                    let move_ = dig_list[watch_index] / 10;

                    dig_list[watch_index] = dig_list[watch_index] % 10;

                    dig_list[watch_index - 1] += move_;
                    watch_index -= 1;
                }

                // println!("i:{:?} j:{:?}  = {:?}", chars2[i], chars1[j], dig_mul_dig);
            }
        }

        // println!("{:?}", dig_list);

        Solution::get_num3(&dig_list)
    }

    pub fn get_num3(nums: &Vec<i32>) -> String {
        let mut res = String::new();
        let mut start = nums.len();

        for i in 0..nums.len() {
            if nums[i] != 0 {
                start = i;
                break;
            }
        }

        if start == nums.len() {
            return "0".to_string();
        }

        for i in start..nums.len() {
            let s = nums[i].to_string();
            res.push_str(&s);
        }

        res
    }
}

// 各种工具函数

// 在rust中，给二维数组每行增加一个“vec”
fn leetcode_testcase_vec2d(input: &str) -> Vec<Vec<i32>> {
    let input = input.trim_matches(|c| c == '[' || c == ']' || c == ',');

    let mut result: Vec<Vec<i32>> = Vec::new();
    for inner_vec_str in input.split("],[").map(|s| s.split(',').collect::<Vec<_>>()) {
        let inner_vec: Vec<i32> = inner_vec_str
            .iter()
            .filter_map(|&x| x.parse().ok())
            .collect();
        result.push(inner_vec);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        let num1 = "12".to_string();

        let num2 = "34".to_string();

        assert_eq!("408".to_string(), Solution::multiply(num1, num2));
    }

    #[test]
    fn it_works2() {
        let num1 = "999".to_string();

        let num2 = "999".to_string();

        assert_eq!("998001".to_string(), Solution::multiply(num1, num2));
    }
}
