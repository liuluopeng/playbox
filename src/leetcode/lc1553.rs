
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::min_days(10));

        assert_eq!(6, Solution::min_days(56));

        assert_eq!(10, Solution::min_days(673));

        assert_eq!(23, Solution::min_days(9209408));

        assert_eq!(25, Solution::min_days(41493528));
    }
}
use std::{collections::HashMap, i32};

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut res = 0;

        let mut record = HashMap::new();

        record.insert(0, 0);
        record.insert(1, 1);

        Self::find(&mut record, n);

        *record.get(&n).unwrap()
    }

    pub fn find(record: &mut HashMap<i32, i32>, remain_origins: i32) -> i32 {
        if record.contains_key(&remain_origins) {
            return *record.get(&remain_origins).unwrap();
        }

        let res2 = 1 + (remain_origins % 2) + Self::find(record, remain_origins / 2);
        let res3 = 1 + (remain_origins % 3) + Self::find(record, remain_origins / 3);

        let res = res2.min(res3);

        record.insert(remain_origins, res);

        res
    }
}
