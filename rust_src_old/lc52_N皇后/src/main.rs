fn main() {
    println!("Hello, world!");

    for i in 1..100 {
        println!("{:?}", Solution::total_n_queens(i));
    }
}

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut count = 0;
        let mut n = n as usize;

        // 构造棋盘
        let mut board = Vec::new();
        for i in 0..n {
            let line = vec![false; n];
            board.push(line);
        }

        let mut col_exists = vec![false; n];

        let mut north_east_exists = vec![false; 2 * n - 1];
        let mut north_west_exists = vec![false; 2 * n - 1];

        Solution::find(
            n,
            &mut board,
            &mut col_exists,
            &mut north_east_exists,
            &mut north_west_exists,
            0,
            &mut count,
        );

        count as i32
    }

    pub fn find(
        n: usize,
        board: &mut Vec<Vec<bool>>,
        col_exists: &mut Vec<bool>,
        ne_exist: &mut Vec<bool>,
        nw_exist: &mut Vec<bool>,
        index: usize, // 放到第几个Queen了.
        count: &mut usize,
    ) {
        if index == n {
            *count += 1;
            return;
        }
        // 试着 把 第index个皇后  放到 j 列
        for j in 0..n {
            if col_exists[j] == false
                && ne_exist[index + j] == false
                && nw_exist[index + n - j - 1] == false
            {
                // 落子的位置是 board[index][j]
                board[index][j] = true;
                col_exists[j] = true;
                ne_exist[index + j] = true;
                nw_exist[index + n - j - 1] = true;
                // 继续往前寻找
                Solution::find(n, board, col_exists, ne_exist, nw_exist, index + 1, count);
                col_exists[j] = false;
                ne_exist[index + j] = false;
                nw_exist[index + n - j - 1] = false;
            }
        }
    }
}
