impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut memo = vec![vec![-1; n]; m];

        Self::find(&grid, m - 1, n - 1, &mut memo);

        // for l in &grid {
        //     println!("{:?} . ", l);
        // }

        // println!();

        // for l in &memo {
        //     println!("{:?}\t", l);
        // }

        memo[m - 1][n - 1]
    }

    // 从grid[0][0]到grid[row][col]达的路径最小值.
    fn find(grid: &Vec<Vec<i32>>, row_idx: usize, col_idx: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        // println!("visit {} {} {}", row_idx, col_idx, grid[row_idx][col_idx]);

        if memo[row_idx][col_idx] != -1 {
            return memo[row_idx][col_idx];
        }

        if row_idx == 0 || col_idx == 0 {
            if row_idx == 0 && col_idx == 0 {
                memo[0][0] = grid[0][0];
                return memo[0][0];
            } else if row_idx == 0 {
                memo[0][col_idx] = Self::find(grid, 0, col_idx - 1, memo) + grid[0][col_idx];

                // println!("0row: {}  from {}  {}", memo[0][col_idx], memo[0][col_idx - 1], grid[0][col_idx]);
                return memo[0][col_idx];
            } else if col_idx == 0 {
                memo[row_idx][0] = Self::find(grid, row_idx - 1, 0, memo) + grid[row_idx][0];
                return memo[row_idx][0];
            }
        }

        let from_up = Self::find(grid, row_idx - 1, col_idx, memo);

        let from_left = Self::find(grid, row_idx, col_idx - 1, memo);

        // println!("从上面 {} {} {}", row_idx - 1, col_idx, from_up);
        // println!("从左面 {} {} {} ", row_idx, col_idx - 1, from_left);

        memo[row_idx][col_idx] = from_left.min(from_up) + grid[row_idx][col_idx];

        Self::find(grid, row_idx - 1, col_idx - 1, memo);

        memo[row_idx][col_idx]
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        let grid = vec_2d_leetcode("[[3,8,6,0,5,9,9,6,3,4,0,5,7,3,9,3],[0,9,2,5,5,4,9,1,4,6,9,5,6,7,3,2],[8,2,2,3,3,3,1,6,9,1,1,6,6,2,1,9],[1,3,6,9,9,5,0,3,4,9,1,0,9,6,2,7],[8,6,2,2,1,3,0,0,7,2,7,5,4,8,4,8],[4,1,9,5,8,9,9,2,0,2,5,1,8,7,0,9],[6,2,1,7,8,1,8,5,5,7,0,2,5,7,2,1],[8,1,7,6,2,8,1,2,2,6,4,0,5,4,1,3],[9,2,1,7,6,1,4,3,8,6,5,5,3,9,7,3],[0,6,0,2,4,3,7,6,1,3,8,6,9,0,0,8],[4,3,7,2,4,3,6,4,0,3,9,5,3,6,9,3],[2,1,8,8,4,5,6,5,8,7,3,7,7,5,8,3],[0,7,6,6,1,2,0,3,5,0,8,0,8,7,4,3],[0,4,3,4,9,0,1,9,7,7,8,6,4,6,9,5],[6,5,1,9,9,2,2,7,4,2,7,2,2,3,7,2],[7,1,9,6,1,2,7,0,9,6,6,4,4,5,1,0],[3,4,9,2,8,3,1,2,6,9,7,0,2,4,2,0],[5,1,8,8,4,6,8,5,2,4,1,6,2,2,9,7]]");
        assert_eq!(83, Solution::min_path_sum(grid));

        // let grid = vec_2d_leetcode("[[1,3,1],[1,5,1],[4,2,1]]");
        // let grid = vec_2d_leetcode("[[1,2],[3,4]");
        // Solution::min_path_sum(grid);
    }
}
