use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut time_counter = -1;
        let mut grid = grid.clone();
        let mut fresh = 0;
        let mut q = vec![];
        // find the first bad orange; count the number of oranges;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    fresh += 1;
                }

                if grid[i][j] == 2 {
                    q.push((i, j));
                }
            }
        }

        while q.len() > 0 {
            time_counter += 1;

            let mut tmp_q = vec![];

            for (idx_i, idx_j) in q {
                // println!("{:?} {:?}", idx_i, idx_j);

                // add 4 direc:
                for (i_delta, j_delta) in [
                    (idx_i.saturating_sub(1), idx_j),
                    (idx_i + 1, idx_j),
                    (idx_i, idx_j.saturating_sub(1)),
                    (idx_i, idx_j + 1),
                ] {
                    if i_delta < grid.len()
                        && j_delta < grid[0].len()
                        && grid[i_delta][j_delta] == 1
                    {
                        fresh -= 1;
                        grid[i_delta][j_delta] = 2;
                        tmp_q.push((i_delta, j_delta));
                    }
                }
            }

            q = tmp_q;
        }

        if fresh == 0 {
            if time_counter > 0 {
                time_counter
            } else {
                0
            }
        } else {
            -1
        }
    }
}
/// 占位 用来引入需要的函数
pub struct Solution2;
impl Solution2 {
    pub fn ppp(root: Option<Rc<RefCell<TreeNode>>>) {}
}

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution, util::leetcode_testcase_vec2d};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let st = "[1,2,3]";
        let root = string2tree(st);

        let st = "[[2,1,1],[1,1,0],[0,1,1]]";
        let v2d = leetcode_testcase_vec2d(st);
        println!("{:?}", Solution::oranges_rotting(v2d));
    }
}
