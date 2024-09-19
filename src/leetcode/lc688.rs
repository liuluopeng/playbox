impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let mut record = vec![vec![vec![-1f64; k + 2]; n]; n];

        let total = Self::find(n, row as usize, column as usize, k, &mut record);
        // println!("total: {}", total);

        // (total as f64) / (8usize.pow(k.try_into().unwrap()) as f64)

        total
    }

    // find(row_idx, col_idx, remain_step)   find()到没有步数的时候的情况总数
    pub fn find(
        n: usize,
        row_idx: usize,
        col_idx: usize,
        remain_setp: usize,
        record: &mut Vec<Vec<Vec<f64>>>,
    ) -> f64 {
        if remain_setp == 0 {
            return 1f64;
        }

        if remain_setp == 1 {
            return 0.125 * Self::get_num(n, row_idx, col_idx).len() as f64;
        }

        if record[row_idx][col_idx][remain_setp] == -1f64 {
            let maybe_list = Self::get_num(n, row_idx, col_idx);
            let mut sum = 0f64;
            for (next_row, next_col) in maybe_list {
                // println!("sum: {}", sum);
                sum += 0.125 * Self::find(n, next_row, next_col, remain_setp - 1, record);
            }

            record[row_idx][col_idx][remain_setp] = sum;
        }

        record[row_idx][col_idx][remain_setp]
    }

    pub fn get_num(n: usize, row_idx: usize, col_idx: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];

        if row_idx >= 2 && col_idx >= 1 {
            res.push((row_idx - 2, col_idx - 1))
        }
        if row_idx >= 2 && col_idx + 1 < n {
            res.push((row_idx - 2, col_idx + 1))
        }
        if row_idx >= 1 && col_idx + 2 < n {
            res.push((row_idx - 1, col_idx + 2))
        }
        if row_idx + 1 < n && col_idx + 2 < n {
            res.push((row_idx + 1, col_idx + 2))
        }
        if row_idx + 2 < n && col_idx + 1 < n {
            res.push((row_idx + 2, col_idx + 1))
        }
        if row_idx + 2 < n && col_idx >= 1 {
            res.push((row_idx + 2, col_idx - 1));
        }
        if row_idx + 1 < n && col_idx >= 2 {
            res.push((row_idx + 1, col_idx - 2));
        }
        if row_idx >= 1 && col_idx >= 2 {
            res.push((row_idx - 1, col_idx - 2));
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
        //         输入: n = 3, k = 2, row = 0, column = 0
        // 输出: 0.0625
        assert_eq!(0.0625, Solution::knight_probability(3, 2, 0, 0));

        //         输入: n = 1, k = 0, row = 0, column = 0
        // 输出: 1.00000

        assert_eq!(1.0000, Solution::knight_probability(1, 0, 0, 0));

        assert_eq!(0.00019, Solution::knight_probability(8, 30, 6, 4));
    }
}
