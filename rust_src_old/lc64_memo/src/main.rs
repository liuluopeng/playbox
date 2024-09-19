use std::mem;

fn main() {
    println!("Hello, world!");

    let st = "[[1,3,1],[1,5,1],[4,2,1]]";
    let st = "[[1,2,3],[4,5,6]]";
    let grid = conv(st);

    println!("{:?}", Solution::min_path_sum(grid));
}
// 在rust中，给二维数组每行增加一个“vec”
fn conv(input: &str) -> Vec<Vec<i32>> {
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
struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;

        // let mut memo_2d: Vec<Vec<i32>> = Vec::new();

        let mut memo_2d_in1line: Vec<i32> = vec![];

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                
                if i == 0 {
                    if j == 0 {
                        memo_2d_in1line.push(grid[i][j]);
                    } else {
                        memo_2d_in1line
                            .push(grid[i][j] + memo_2d_in1line[memo_2d_in1line.len() - 1])
                    }
                } else {
                    // i != 0
                    let from_left = memo_2d_in1line[memo_2d_in1line.len() - 1 ];
                    let from_up = memo_2d_in1line[memo_2d_in1line.len()  - grid[0].len()];

                    let min = from_left.min(from_up);

                    memo_2d_in1line.push(min + grid[i][j])
                }
            }
        }

        println!("{:?}", memo_2d_in1line);
        
        memo_2d_in1line[memo_2d_in1line.len() - 1]
    }
}
