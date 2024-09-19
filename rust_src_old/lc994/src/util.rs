

// 各种工具函数

// 在rust中，给二维数组每行增加一个“vec”
pub fn leetcode_testcase_vec2d(input: &str) -> Vec<Vec<i32>> {
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
