impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        fn find(
            row_idx_start: usize,
            col_idx_start: usize,
            memo: &mut Vec<Vec<i32>>,
            matrix: &Vec<Vec<i32>>,
        ) -> i32 {
            if memo[row_idx_start][col_idx_start] == -1 {
                let me = matrix[row_idx_start][col_idx_start];

                // 上下左右和me比较.
                /*
                               up
                         left   me  right
                               down
                */

                let mut to_up = 1;
                if row_idx_start >= 1 {
                    if matrix[row_idx_start - 1][col_idx_start] > me {
                        to_up += find(row_idx_start - 1, col_idx_start, memo, matrix);
                    }
                }

                let mut to_left = 1;
                if col_idx_start >= 1 {
                    if matrix[row_idx_start][col_idx_start - 1] > me {
                        to_left += find(row_idx_start, col_idx_start - 1, memo, matrix);
                    }
                }

                let mut to_right = 1;
                if col_idx_start < matrix[0].len() - 1 {
                    if matrix[row_idx_start][col_idx_start + 1] > me {
                        to_right += find(row_idx_start, col_idx_start + 1, memo, matrix);
                    }
                }

                let mut to_down = 1;
                if row_idx_start < matrix.len() - 1 {
                    if matrix[row_idx_start + 1][col_idx_start] > me {
                        to_down += find(row_idx_start + 1, col_idx_start, memo, matrix);
                    }
                }

                // println!("{} {} {} {}", to_down, to_left, to_right, to_up);
                memo[row_idx_start][col_idx_start] = to_down.max(to_up).max(to_left).max(to_right);
            }

            memo[row_idx_start][col_idx_start]
        }

        let m = matrix.len();
        let n = matrix[0].len();

        // memo[i][j]含义:从matrix[i][j]出发, 一直到不能递增, 能达到的最大个数.
        let mut memo = vec![vec![-1; n]; m];

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                let one_res = find(i, j, &mut memo, &matrix);
                res = res.max(one_res);
            }
        }

        for l in &memo {
            println!("line: {:?}", l);
        }

        res
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            Solution::longest_increasing_path(vec_2d_leetcode("[[9,9,4],[6,6,8],[2,1,1]]"))
        );
    }
}
