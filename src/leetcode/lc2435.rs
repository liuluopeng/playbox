impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut record = vec![vec![vec![0; k + 1]; grid[0].len()]; grid.len()];
        let mut exist = vec![vec![vec![false; k + 1]; grid[0].len()]; grid.len()];

        Self::find(
            &grid,
            &mut record,
            grid.len() - 1,
            grid[0].len() - 1,
            k,
            0,
            &mut exist,
        );

        record[grid.len() - 1][grid[0].len() - 1][0] as i32
    }

    pub fn find(
        grid: &Vec<Vec<i32>>,
        record: &mut Vec<Vec<Vec<usize>>>,
        row_now: usize,
        col_now: usize,
        k: usize,
        should_remain: usize, // k_: 应该的余数
        exist: &mut Vec<Vec<Vec<bool>>>,
    ) -> usize {
        if row_now == 0 && col_now == 0 {
            println!(
                "gird[0][0]: {}  remain: {}  k: {}  res: {}",
                grid[0][0],
                should_remain,
                k,
                ((grid[0][0] as usize) % k + should_remain) % k
            );

            if ((grid[0][0] as usize) % k + should_remain) % k == 0 {
                record[0][0][should_remain] = 1;
                return 1;
            }
        }

        if exist[row_now][col_now][should_remain] == false {
            let new_k_ = (should_remain + grid[row_now][col_now] as usize) % k;

            let mut from_up = 0;
            let mut from_left = 0;

            if row_now >= 1 {
                from_up = Self::find(grid, record, row_now - 1, col_now, k, new_k_, exist);
            }

            if col_now >= 1 {
                from_left = Self::find(grid, record, row_now, col_now - 1, k, new_k_, exist);
            }

            let res = (from_left + from_up) % 1000000007;

            record[row_now][col_now][should_remain] = res;
            exist[row_now][col_now][should_remain] = true;
        }

        record[row_now][col_now][should_remain]
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        let grid = vec_2d_leetcode("[[5,2,4],[3,0,5],[0,7,2]]");
        let k = 3;
        assert_eq!(2, Solution::number_of_paths(grid, k));

        // gird[0][0]: 5  remain: 1  k: 3
        // gird[0][0]: 5  remain: 3  k: 3
        // gird[0][0]: 5  remain: 1  k: 3

        let grid = vec_2d_leetcode("[[0]]");
        let k = 1;
        assert_eq!(1, Solution::number_of_paths(grid, k));
    }
}
