use termion::{color, style};

fn main() {
    println!("Hello, world!");

    println!(
        "{}{}{}{} ",
        color::Bg(color::Red),
        style::Bold,
        "hh",
        style::Reset
    );

    // let input = r#"[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]"#;
    // let mut board = string_to_vec(input);

    // for line in &board {
    //     println!("{:?}", line);
    // }

    // solve_sudoku(&mut board);

    // 构造出所有基础的数独
    let mut board = vec![vec![0; 9]; 9];
    board[0] = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let un_set_index_list: Vec<usize> = (9..81).collect(); // 总计81-9个72个

    // println!("{:?}", un_set_index_list);

    ppp_board(&board);

    let mut res = vec![];
    find_ess_solution(&mut board, &un_set_index_list, &mut 0, &mut res, &mut 0);

    // 开始打印结果
    println!("以下是所有可能的数独",);
    for b in res {
        ppp_board(&b);
    }
}

/// .找出多个数独的解
fn find_ess_solution(
    board: &mut Vec<Vec<usize>>,
    un_set_list: &Vec<usize>,
    now_index: &mut usize,
    res: &mut Vec<Vec<Vec<usize>>>,
    count: &mut usize,
) {
    // println!("{:?}", now_index);

    if *now_index == 72 {
        // 结束

        *count += 1;
        println!("{:?}", *count);
        pppp_board(&board);
        // ppp_board(&board);
        // res.push(board.to_vec());

        return;
    }

    let (i, j) = get_i_j(un_set_list[*now_index]);

    for num in 1..10 {
        // num: 1~9
        if is_valid(&board, i, j, num) {
            board[i][j] = num;

            // 打印一下 当前有效 的 数独:
            // ppp_board(&board);

            // 继续寻找
            *now_index += 1;
            find_ess_solution(board, un_set_list, now_index, res, count);

            board[i][j] = 0;
            *now_index -= 1;
        } else {
            continue;
        }
    }
}

// 只打印数字
fn pppp_board(board: &Vec<Vec<usize>>) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", board[i][j]);
        }
    }
    print!("\n");
}

// 看看当前的数独有没有错误 刚刚新填入了
fn is_valid(board: &Vec<Vec<usize>>, i: usize, j: usize, num: usize) -> bool {
    let not_repeat = num;

    // 同一行:  board[i][0] 到  board[i][8]
    for k in 0..9 {
        if board[i][k] == not_repeat {
            return false;
        }
    }

    // col:
    for k in 0..9 {
        if board[k][j] == not_repeat {
            return false;
        }
    }

    //
    // let d1_index = i * 9 + j;

    for k in 0..9 {
        if board[3 * (i / 3) + k / 3][3 * (j / 3) + k % 3] == not_repeat {
            // 来自:  【【回溯37】解数独】 https://www.bilibili.com/video/BV15z4y1h7L9/?share_source=copy_web&vd_source=5a2d7ecacd3ba507f3c18fde1192c687

            return false;
        }
    }

    true
}

//
fn get_sub_block_start(i: usize, j: usize) -> usize {
    let zoom_out_i = i / 3;
    let zoom_out_j = j / 3;

    // zoom_out_i * 3
    zoom_out_i + zoom_out_j
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

// 打印当前的数独
pub fn pp_board(board: &Vec<Vec<usize>>, curr_index: usize, un_set_list: &Vec<usize>) {
    let mut already_exist_list = vec![true; 81];
    for &i in un_set_list {
        already_exist_list[i] = false;
    }

    for i in 0..9 {
        for j in 0..9 {
            let index = i * 9 + j;
            if already_exist_list[index] {
                print!(
                    "{}{}{}{}{}",
                    color::Bg(color::Blue),
                    style::Bold,
                    board[i][j],
                    " ",
                    style::Reset
                )
            } else {
                if index == curr_index {
                    print!(
                        "{}{}{}{}{}",
                        color::Bg(color::Red),
                        style::Bold,
                        // board[i][j],
                        " ",
                        " ",
                        style::Reset
                    )
                } else if index < curr_index {
                    // 当前处理的之前已经新填入的
                    print!(
                        "{}{}{}{}{}",
                        color::Bg(color::LightYellow),
                        style::Bold,
                        board[i][j],
                        " ",
                        style::Reset
                    )
                } else {
                    // 之后的, 还没填写, 是空的
                    print!(
                        "{}{}{}{}{}",
                        color::Bg(color::LightGreen),
                        style::Bold,
                        // board[i][j],
                        " ",
                        " ",
                        style::Reset
                    )
                }
            }
        }

        println!();
    }

    println!();
}
// 打印当前的数独
pub fn ppp_board(board: &Vec<Vec<usize>>) {
    for i in 0..9 {
        for j in 0..9 {
            let index = i * 9 + j;

            print!(
                "{}{}{}{}{}",
                color::Bg(color::LightYellow),
                style::Bold,
                board[i][j],
                " ",
                style::Reset,
            )
        }

        println!();
    }

    println!();
}
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

    solve_sudoku_usize(&mut board_usize);

    for i in 0..9 {
        for j in 0..9 {
            board[i][j] = (board_usize[i][j] as u8 + 48) as char;
        }
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
                sub_exists[get_sub_index(i, j)][index_num] = true;
            } else {
                un_set_list.push(i * 9 + j);
            }
        }
    }

    // println!("un set list: {:?}", un_set_list);

    // 开始解题
    find(
        board,
        &mut line_exists,
        &mut col_exists,
        &mut sub_exists,
        0,
        &un_set_list,
        &mut 0,
    );

    pp_board(board, 80, &un_set_list)
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

    // 彩色的方式 打印 数独
    pp_board(board, un_set_list[*seted_index], un_set_list);

    let (i, j) = get_i_j(un_set_list[*seted_index]);

    let sub_index = get_sub_index(i, j);

    let mut space = String::from("");

    for k in 0..depth {
        space += "  ";
    }

    let maybe_list = get_maybe_list(&line_exists, &col_exists, &sub_exists, i, j, sub_index);

    for atmpt in maybe_list.clone() {
        if line_exists[i][atmpt - 1] == false
            && col_exists[j][atmpt - 1] == false
            && sub_exists[sub_index][atmpt - 1] == false
        {
            board[i][j] = atmpt;
            line_exists[i][atmpt - 1] = true;
            col_exists[j][atmpt - 1] = true;
            sub_exists[sub_index][atmpt - 1] = true;

            *seted_index += 1;
            if find(
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
