pub struct Solution;
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;

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
    use crate::solution::{leetcode_testcase_vec2d, Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let str = "[[1,2],[2,3],[3,4],[1,3]]";
        let input = leetcode_testcase_vec2d(str);
        assert_eq!(Solution::erase_overlap_intervals(input), 1);
    }
}
