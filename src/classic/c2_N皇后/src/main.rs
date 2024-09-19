fn main() {
    println!("Hello, world!");

    let n = 1; // 棋盘大小
}

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![];

        let mut chessboards = vec![];

        let mut chessboard = vec![vec![0; n as usize]; n as usize];

        Self::find(&mut chessboards, &mut chessboard, 0, n as usize);

        res
    }

    pub fn find(
        res: &mut Vec<Vec<Vec<i32>>>,
        now: &mut Vec<Vec<i32>>,
        now_index: usize,
        size: usize,
    ) {
        if now_index == size {
            // 找到了一种排布
            res.push(now.to_vec());
        }

        for i in now_index..size { // 在每一行放一个。
            
            

        }
    }
}
