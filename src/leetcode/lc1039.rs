impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut record = vec![vec![-1; values.len()]; values.len()];

        Self::find(&values, 0, values.len() - 1, &mut record)
    }

    // i -> j 是顺时针.
    pub fn find(
        values: &Vec<i32>,
        i_bottom_idx: usize,
        j_bottom_idx: usize,
        record: &mut Vec<Vec<i32>>,
    ) -> i32 {
        // println!("{:?} {:?}", i_bottom_idx, j_bottom_idx);

        if i_bottom_idx == j_bottom_idx {
            return 0;
        }

        // base case : 原始的凸多边形的边, 不能构成三角形.
        if (values.len() + j_bottom_idx - i_bottom_idx) % values.len() == 1 {
            return 0;
        }

        // base case : 相差只差一个的时候, 只能构成唯一的三角形.
        if (values.len() + j_bottom_idx - i_bottom_idx) % values.len() == 2 {
            let between_idx = (i_bottom_idx + 1) % values.len();
            return values[i_bottom_idx] * values[between_idx] * values[j_bottom_idx];
        }

        if record[i_bottom_idx][j_bottom_idx] == -1 {
            let mut res = i32::MAX;

            let counter = (values.len() + j_bottom_idx - i_bottom_idx - 1) % values.len();
            // println!("k: {:?}", counter);

            for k in 1..=counter {
                let top_idx = (k + i_bottom_idx) % values.len();

                let mul = values[i_bottom_idx] * values[j_bottom_idx] * values[top_idx];

                res = res.min(
                    mul + Self::find(values, i_bottom_idx, top_idx, record)
                        + Self::find(values, top_idx, j_bottom_idx, record),
                );
            }

            record[i_bottom_idx][j_bottom_idx] = res;
        }

        record[i_bottom_idx][j_bottom_idx]
    }
}
use std::i32;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            13,
            Solution::min_score_triangulation(vec![1, 3, 1, 4, 1, 5])
        );

        assert_eq!(
            153657,
            Solution::min_score_triangulation(vec![
                38, 76, 69, 32, 24, 35, 82, 30, 86, 77, 92, 3, 35, 20, 84, 67, 23, 58, 94, 10
            ])
        );
    }
}
