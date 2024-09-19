fn main() {
    println!("Hello, world!");

    let n = 2;
    let n = 3;
    let n = 4;
    // let n = 5;
    // let n = 6;
    let n = 8;
    // let n = 9;
    let boards = Solution::solve_n_queens(n);

    println!("{:?}", boards);
}

struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n: usize = n as usize;
        let mut res = Vec::new();

        let mut boards = Vec::new();

        let mut board = Vec::new();
        // 构造棋盘 空格代表没棋子
        for i in 0..n {
            let mut line = Vec::new();
            for j in 0..n {
                line.push(0);
            }
            board.push(line);
        }

        Solution::find(
            &mut boards,
            &mut board,
            0,
            0,
            0,
            0,
            &mut vec![false; n],
            &mut vec![false; n],
        );

        // for b in boards.clone() {
        //     Solution::p_board(&b);
        // }
        // println!("{:?} 个结果", boards.len());


        // 把 0 1 的 棋盘  变成  ...Q....
        Solution::d3_d2(&boards, &mut res);

        res
    }

    pub fn find(
        boards: &mut Vec<Vec<Vec<i32>>>,
        board_now: &mut Vec<Vec<i32>>,
        index: usize,
        start: usize,
        start_i: usize,
        start_j: usize,
        row_exist: &mut Vec<bool>,
        col_exist: &mut Vec<bool>,
    ) {
        // 先解决: 4个棋子在棋盘上一共有多少排放的方式.
        let n = board_now.len();

        if index == n {
            // 打印一个结果
            // Solution::p_board(board_now);
            // boards.push(board_now.to_vec());
            // return;

            // 检查是否符合规则
            if Solution::check(board_now) {
                boards.push(board_now.to_vec());
            }

            return;
        }

        // for i in 0..n {
        //     for j in 0..n {

        //         if board_now[i][j] == 0 {
        //             // println!("{:?} {:?}", i, j);
        //             board_now[i][j] = 1;

        //             Solution::find(boards, board_now, index + 1, i, j);
        //             // 恢复
        //             board_now[i][j] = 0;
        //         }

        //     }
        // }

        for inde_1d in start..n * n {
            let i = inde_1d / n;
            let j = inde_1d % n;

            if board_now[i][j] == 0  && row_exist[i] == false  && col_exist[j] == false {
                // println!("{:?} {:?}", i, j);

                board_now[i][j] = 1;

                // 把 i j 加入不可选取的名单
                row_exist[i] = true;
                col_exist[j] = true;


                Solution::find(
                    boards,
                    board_now,
                    index + 1,
                    inde_1d,
                    i,
                    j,
                    row_exist,
                    col_exist,
                );
                // 恢复
                board_now[i][j] = 0;
                row_exist[i] = false;
                col_exist[j] = false;
            }
        }
    }

    pub fn p_board(board: &Vec<Vec<i32>>) {
        for line in board {
            println!("{:?}", line);
        }
        println!();
    }

    pub fn check(board: &Vec<Vec<i32>>) -> bool {
        // 检查个数
        let mut count = 0;
        let mut n = board.len();
        // 行 检查器
        let mut line_checker = vec![0; n];
        let mut col_checker = vec![0; n];

        for i in 0..n {
            for j in 0..n {
                if board[i][j] == 1 {
                    count += 1;
                    line_checker[i] += 1;
                    col_checker[j] += 1;
                }
            }
        }

        if count != n {
            return false;
        }

        for i in line_checker {
            if i != 1 {
                return false;
            }
        }

        for i in col_checker {
            if i != 1 {
                return false;
            }
        }

        // 检查对角线  方向: 东北 西南
        let mut NE2SW = vec![];
        // 加入第一行
        for i in 0..n {
            NE2SW.push(vec![0, i]);
        }
        // 加入最后一列
        for i in 1..n {
            NE2SW.push(vec![i, n - 1]);
        }

        // println!("{:?}", NE2SW);

        let mut NE_counter = vec![0; NE2SW.len()];
        for index in 0..NE2SW.len() {
            let mut i = NE2SW[index][0];
            let mut j = NE2SW[index][1];

            while i < n {
                if board[i][j] == 1 {
                    NE_counter[index] += 1;
                }
                i += 1;

                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }

        // println!("{:?}", NE_counter);
        for i in NE_counter {
            if i > 1 {
                return false;
            }
        }

        // 检查对角线  方向: 西北 东南
        let mut NW2SE = vec![];

        for i in 0..n {
            NW2SE.push(vec![i, 0]);
        }

        for i in 1..n {
            NW2SE.push(vec![0, i]);
        }
        let mut NW_counter = vec![0; NW2SE.len()];
        for index in 0..NE2SW.len() {
            let mut i = NW2SE[index][0];
            let mut j = NW2SE[index][1];

            while i < n && j < n {
                if board[i][j] == 1 {
                    NW_counter[index] += 1;
                }
                i += 1;

                j += 1;
            }
        }

        for i in NW_counter {
            if i > 1 {
                return false;
            }
        }

        true
    }


    pub fn d3_d2(boards: &Vec<Vec<Vec<i32>>>, res: &mut Vec<Vec<String>>) {

        for board in boards {
            let mut one_res = vec![];


            for i in board {
                let mut st = String::new();
                for j in i {
                    if *j == 0 {
                        st.push('.');
                    }
                    if *j == 1 {
                        st.push('Q');
                    }
                }
                one_res.push(st);
            }

            res.push(one_res)
        }

    }
}
