use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(8, Solution::count_paths(vec_2d_leetcode("[[1,1],[3,4]]")));
    }
}

impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut record = vec![vec![None; n]; m];

        let mut accu = 0;
        for i in 0..m {
            for j in 0..n {
                let res = Self::find(i, j, &grid, &mut record) % 1000000007;
                // println!("i  j  {}   {}  res: {}", i, j, res);
                accu += res;
                accu %= 1000000007;
            }
        }

        accu as i32
    }

    // find row col : 从(row,col)出发的路径总数. 路径是递增的路径.

    /*

            1    3
        0   1    2   3
            1    3

       find(中间的1) = find(0) + find(2)
       find(3) = 自己作为长度1的路径
    */

    fn find(
        row_start: usize,
        col_start: usize,
        grid: &Vec<Vec<i32>>,
        record: &mut Vec<Vec<Option<usize>>>,
    ) -> usize {
        if let Some(r) = record[row_start][col_start] {
            return r;
        } else {
            let modder = 1000000007;

            let mut res = 1;

            if row_start >= 1 && grid[row_start - 1][col_start] > grid[row_start][col_start] {
                // 可以往上走
                res += Self::find(row_start - 1, col_start, grid, record);
            }

            if row_start + 1 < grid.len()
                && grid[row_start + 1][col_start] > grid[row_start][col_start]
            {
                res += Self::find(row_start + 1, col_start, grid, record);
            }

            if col_start >= 1 && grid[row_start][col_start - 1] > grid[row_start][col_start] {
                res += Self::find(row_start, col_start - 1, grid, record);
            }
            if col_start + 1 < grid[0].len()
                && grid[row_start][col_start + 1] > grid[row_start][col_start]
            {
                res += Self::find(row_start, col_start + 1, grid, record);
            }

            record[row_start][col_start] = Some(res % modder);
            return res;
        }
    }
}
