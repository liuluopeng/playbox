use core::borrow;

struct Solution {}

// 在rust中，给二维数组每行增加一个“vec”
fn conv_2di32list_to_vec(input: &str) -> Vec<Vec<i32>> {
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

fn conv_2dstr_list_to_vec(input: &str) -> Vec<Vec<char>> {
    let input = input.trim_matches(|c| c == '[' || c == ']' || c == ',');

    let mut result: Vec<Vec<char>> = Vec::new();
    for inner_vec_str in input.split("],[").map(|s| s.split(',').collect::<Vec<_>>()) {
        let inner_vec: Vec<char> = inner_vec_str
            .iter()
            .filter_map(|&x| {
                if x == "\".\"" {
                    Some('.')
                } else {
                    let dig = Some(&x[1..x.len() - 1]);
                    let num: i32 = dig.unwrap().parse().expect("msg");
                    std::char::from_digit(num as u32, 10)
                }
            })
            .collect();
        result.push(inner_vec);
    }
    result
}

fn main() {
    let st = r#"[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]"#;

    let board = conv_2dstr_list_to_vec(st);
    println!("{:?}", board[0]);
    Solution::is_valid_sudoku(board);
}

impl Solution {
    pub fn find_block(i: usize, j: usize) -> usize {
        let block_i = i / 3;
        let block_j = j / 3;
        return block_i * 3 + block_j;
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_exsit = vec![vec![0; 9]; 9];
        let mut col_exsit = vec![vec![0; 9]; 9];
        let mut sq_exsit = vec![vec![0; 9]; 9];

        for row_index in 0..9 {
            for col_index in 0..9 {
                let num_char = board[row_index][col_index];
                if num_char == '.' {
                    continue;
                } else {
                    let num: usize = (num_char as u8 - 48) as usize;
                    // println!("{} {} {}", row_index, col_index, num);
                    row_exsit[row_index][num - 1] += 1;
                    col_exsit[col_index][num - 1] += 1;
                    let block_index = Self::find_block(row_index, col_index);
                    sq_exsit[block_index][num - 1] += 1;

                    if row_exsit[row_index][num - 1] > 1
                        || col_exsit[col_index][num - 1] > 1
                        || sq_exsit[block_index][num - 1] > 1
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}
