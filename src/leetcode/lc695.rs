impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut max_now = 0;

        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let mut a_res = 0;
                    Self::find(&grid, &mut a_res, &mut visited, (i, j));
                    if a_res > max_now {
                        max_now = a_res;
                    }
                }
            }
        }

        max_now
    }

    pub fn find(
        grid: &Vec<Vec<i32>>,
        accu: &mut i32,
        visited: &mut Vec<Vec<bool>>,
        now_idx: (usize, usize),
    ) {
        if grid[now_idx.0][now_idx.1] == 1 && visited[now_idx.0][now_idx.1] == false {
            visited[now_idx.0][now_idx.1] = true;
            *accu += 1;

            if now_idx.0 >= 1 {
                Self::find(grid, accu, visited, (now_idx.0 - 1, now_idx.1));
            }
            if now_idx.0 + 1 < grid.len() {
                Self::find(grid, accu, visited, (now_idx.0 + 1, now_idx.1));
            }
            if now_idx.1 >= 1 {
                Self::find(grid, accu, visited, (now_idx.0, now_idx.1 - 1));
            }
            if now_idx.1 + 1 < grid[0].len() {
                Self::find(grid, accu, visited, (now_idx.0, now_idx.1 + 1));
            }
        } else {
            return;
        }
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(6, Solution::max_area_of_island(vec_2d_leetcode("[[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]")));
    }
}
