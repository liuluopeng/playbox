struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let res = vec![];

        let mut chessboard = vec![];
        for i in 0..n {
            let mut line = vec![];
            for j in 0..n {
                line.push(0)
            }
            chessboard.push(line);
        }

        let mut result: Vec<Vec<Vec<i32>>> = vec![];
        result.push(chessboard.clone());

        Self::find(&mut result, &mut chessboard, &mut vec![0, 0]);

        println!("结果的总数 {}", result.len());

        for r in result {
            for line in r {
                println!("{:?}", line);
            }
            println!("");
        }

        res
    }

    pub fn find(
        result: &mut Vec<Vec<Vec<i32>>>,
        board: &mut Vec<Vec<i32>>,
        start_index: &mut Vec<i32>,
    ) {

        println!("现在放第一个棋子的坐标 {:?}", start_index);

        if start_index[0] == board.len()  as i32 - 1 || start_index[1] == board.len() as i32 - 1{
            result.push(board.to_vec());
            return;
        }


        for i in 0..board.len() {
            board[i][0] = 1;

            start_index[0] = i as i32 + 1;
            Self::find(result, board, start_index);

            start_index[0] = i as i32 - 1;
            board[i][0] = 0;
        }
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", Solution::solve_n_queens(4))
}
