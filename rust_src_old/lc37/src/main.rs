fn main() {
    println!("Hello, world!");

    let input = r#"[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]"#;
    let mut board = string_to_vec(input);

    for line in &board {
        println!("{:?}", line);
    }

    Solution::solve_sudoku(&mut board);
}

fn string_to_vec(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            if ch.is_digit(10) || ch == '.' {
                row.push(ch);
            }
        }
        // result.push(row);

        // 每隔9个
        let mut i = 0;
        let mut turn = 0;
        let mut l = vec![];
        while i < row.len() {
            l.push(row[i]);

            turn += 1;
            i += 1;

            if turn == 9 {
                result.push(l.clone());
                turn = 0;
                l.clear();
            }
        }
    }

    result
}

struct Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board_usize = vec![];

        for line in board.clone() {
            let mut one_line = vec![];
            for b in line {
                if b == '.' {
                    one_line.push(0);
                } else {
                    one_line.push(b as usize - 48);
                }
            }
            board_usize.push(one_line);
        }

        Solution::solve_sudoku_usize(&mut board_usize);

        for i in 0..9 {
            for j in 0..9 {
                board[i][j] =( board_usize[i][j] as u8 + 48)  as char;
            }
        }

        println!();
        for line in board.clone() {
            println!("{:?}", line);
        }
    }

    pub fn solve_sudoku_usize(board: &mut Vec<Vec<usize>>) {
        let mut line_exists: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut col_exists: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut sub_exists: Vec<Vec<bool>> = vec![vec![false; 9]; 9];

        let mut un_set_list = vec![];
        // 统计已经落位的格子
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != 0 {
                    let index_num = board[i][j] - 1;

                    line_exists[i][index_num] = true; // index+1已经在某一行存在了.
                    col_exists[j][index_num] = true; //
                    sub_exists[Solution::get_sub_index(i, j)][index_num] = true;
                } else {
                    un_set_list.push(i * 9 + j);
                }
            }
        }

        // println!("un set list: {:?}", un_set_list);

        // 开始解题
        Solution::find(
            board,
            &mut line_exists,
            &mut col_exists,
            &mut sub_exists,
            0,
            &un_set_list,
            &mut 0,
        );
    }

    pub fn get_sub_index(i: usize, j: usize) -> usize {
        let i_zoom = i / 3;
        let j_zoom = j / 3;
        i_zoom * 3 + j_zoom
    }

    pub fn get_i_j(index: usize) -> (usize, usize) {
        let i = index / 9;
        let j = index % 9;
        (i, j)
    }

    pub fn find(
        board: &mut Vec<Vec<usize>>,
        line_exists: &mut Vec<Vec<bool>>,
        col_exists: &mut Vec<Vec<bool>>,
        sub_exists: &mut Vec<Vec<bool>>,
        depth: usize,
        un_set_list: &Vec<usize>,
        seted_index: &mut usize,
    ) -> bool {
        if *seted_index == un_set_list.len() {
            return true;
        }

        let (i, j) = Solution::get_i_j(un_set_list[*seted_index]);

        let sub_index = Solution::get_sub_index(i, j);

        let mut space = String::from("");
        for k in 0..depth {
            space += "  ";
        }
        // println!("{}现在的位置: {} {}   {:?}", space, i, j, board[i][j]);

        // for line in board.clone() {
        //     println!("{}{:?}", space, line);
        // }

        let maybe_list =
            Solution::get_maybe_list(&line_exists, &col_exists, &sub_exists, i, j, sub_index);

        // if maybe_list.len() == 0 {
        //     println!("{}是空的,但是不能填任何数字 {} {}", space, i, j);
        // }

        for atmpt in maybe_list.clone() {
            if line_exists[i][atmpt - 1] == false
                && col_exists[j][atmpt - 1] == false
                && sub_exists[sub_index][atmpt - 1] == false
            {
                // println!(
                //     "{}{:?}  {:?} 尝试填入: {:?}  还可以填写{:?}",
                //     space, i, j, atmpt, maybe_list
                // );

                board[i][j] = atmpt;
                line_exists[i][atmpt - 1] = true;
                col_exists[j][atmpt - 1] = true;
                sub_exists[sub_index][atmpt - 1] = true;

                *seted_index += 1;
                if Solution::find(
                    board,
                    line_exists,
                    col_exists,
                    sub_exists,
                    depth + 1,
                    un_set_list,
                    seted_index,
                ) {
                    return true;
                } else {
                    // atmpt没有解, 回退
                    board[i][j] = 0;
                    line_exists[i][atmpt - 1] = false;
                    col_exists[j][atmpt - 1] = false;
                    sub_exists[sub_index][atmpt - 1] = false;
                    *seted_index -= 1;
                }
            }
        }

        // println!();
        // println!("{}无解 {}   {} ", space, i, j);
        return false;
    }

    pub fn get_maybe_list(
        ex1: &Vec<Vec<bool>>,
        ex2: &Vec<Vec<bool>>,
        ex3: &Vec<Vec<bool>>,
        i: usize,
        j: usize,
        sub_index: usize,
    ) -> Vec<usize> {
        let mut res = vec![];

        for k in 0..9 {
            if ex1[i][k] == false && ex2[j][k] == false && ex3[sub_index][k] == false {
                res.push(k + 1);
            }
        }

        res
    }
}
