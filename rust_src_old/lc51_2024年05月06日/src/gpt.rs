fn main() {
    let n = 2; // 棋盘大小
    let k = 2; // 要放置的棋子数量
    let mut board = vec![vec![0; n]; n]; // 初始化棋盘
    let mut solutions = Vec::new();
    backtrack(&mut board, k, 0, &mut solutions);
    println!("Total ways to place the pieces: {}", solutions.len());
    for solution in solutions {
        println!("{:?}", solution);
    }
}

fn backtrack(board: &mut Vec<Vec<i32>>, k: i32, start: usize, solutions: &mut Vec<Vec<Vec<i32>>>) {
    if k == 0 {
        // 已经放置了所需数量的棋子，保存当前棋盘状态
        solutions.push(board.clone());
        return;
    }

    // 棋盘是一维展开的，对于当前start位置，转换为二维位置
    let n = board.len();
    let row = start / n;
    let col = start % n;

    for i in start..(n*n) {
        // 放置棋子
        board[row][col] = 1;
        // 继续放置下一个棋子
        backtrack(board, k - 1, i + 1, solutions);
        // 撤销操作
        board[row][col] = 0;

        // 找到下一个空位
        if col < n - 1 {
            board[row][col] = 0; // 确保当前位置为0
            // 尝试下一列
            backtrack(board, k, i + 1, solutions);
        } else if row < n - 1 {
            // 尝试下一行的起始处
            backtrack(board, k, (row + 1) * n, solutions);
        }
    }
}