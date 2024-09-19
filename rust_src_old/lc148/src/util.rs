

// 各种工具函数

use std::{fs::File, io::{self, BufRead}, path::Path};

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



pub fn read_numbers_from_file(file_path: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if let Some((start, end)) = trimmed.find('[').and_then(|start| {
            trimmed.rfind(']').map(|end| (start, end))
        }) {
            let content = &trimmed[start + 1..end];
            for num_str in content.split(',') {
                if let Ok(num) = num_str.parse::<i32>() {
                    numbers.push(num);
                }
            }
        }
    }

    Ok(numbers)
}