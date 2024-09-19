impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes.clone();
        // 定义比较函数
        envelopes.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut heights = vec![];
        for i in envelopes {
            heights.push(i[1]);
        }

        let mut ends = vec![];
        for i in 0..heights.len() {
            if let Some(target_idx) = Self::get_replace_idx(&ends, heights[i]) {
                ends[target_idx] = heights[i];
            } else {
                ends.push(heights[i]);
            }
        }

        ends.len() as i32
    }

    // 寻找一个下标, ends[l_idx] >= val
    fn get_replace_idx(ends: &Vec<i32>, val: i32) -> Option<usize> {
        let mut l_idx = 0;

        let mut r_border = ends.len();

        while l_idx < r_border {
            let mid_idx = (l_idx + r_border) / 2;

            if ends[mid_idx] < val {
                l_idx = mid_idx + 1;
            } else {
                r_border = mid_idx;
            }
        }

        if l_idx < ends.len() {
            Some(l_idx)
        } else {
            None
        }
    }
}
use std::env;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        /*
                输入：envelopes = [[5,4],[6,4],[6,7],[2,3]]
        输出：3
        解释：最多信封的个数为 3, 组合为: [2,3] => [5,4] => [6,7]。 */

        assert_eq!(
            3,
            Solution::max_envelopes(vec_2d_leetcode("[[5,4],[6,4],[6,7],[2,3]]"))
        );
    }
}
