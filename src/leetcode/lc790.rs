impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut res = 0;

        let mut record = HashMap::new();
        res = Self::raw_slove(n, false, &mut record);

        res as i32
    }

    pub fn raw_slove(
        remain_n: i32,
        has_half: bool,
        record: &mut HashMap<(i32, bool), usize>,
    ) -> usize {
        // has_half:前面遗留了一个方块
        if remain_n == 0 {
            if has_half {
                return 0;
            } else {
                // 完整
                return 1;
            }
        }

        if remain_n == 1 {
            /* has half:
               * ^
               ^ ^
            */

            /*  has no half:
                ^
                ^
            */
            return 1;
        }
        let mut res = 0;
        if record.contains_key(&(remain_n, has_half)) == false {
            if has_half {
                // 前面有一个留下来交给n:

                res = Self::raw_slove(remain_n - 1, true, record); // 横着摆放一个多米诺
                res += Self::raw_slove(remain_n - 1, false, record); // 遗留的和n组成一个tomino
            } else {
                // 前面是完整的:
                res = Self::raw_slove(remain_n - 1, false, record);
                res += Self::raw_slove(remain_n - 2, false, record);
                res += 2 * Self::raw_slove(remain_n - 2, true, record);
            }

            record.insert((remain_n, has_half), res % 1000000007);
        }

        *record.get(&(remain_n, has_half)).unwrap()
    }
}

use std::{collections::HashMap, hash::Hash};

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::num_tilings(3));

        assert_eq!(312342182, Solution::num_tilings(30));
    }
}
