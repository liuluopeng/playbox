impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut record = vec![vec![-1; s.len()]; s.len()];

        Self::find(&s, 0, s.len() - 1, &mut record)
    }

    // 得出答案: s[l..r] 的让字符串成为回文串 的最少插入次数
    fn find(s: &Vec<char>, l: usize, r: usize, record: &mut Vec<Vec<i32>>) -> i32 {
        if l == r {
            return 0;
        }

        if l + 1 == r {
            if s[l] == s[r] {
                return 0;
            } else {
                // s[l] != s[r]
                return 1;
            }
        }

        let mut res = i32::MAX;

        if record[l][r] == -1 {
            if s[l] == s[r] {
                res = res.min(Self::find(s, l + 1, r - 1, record));
            } else {
                res = res.min(Self::find(s, l + 1, r, record) + 1);
                res = res.min(Self::find(s, l, r - 1, record) + 1);
            }

            record[l][r] = res;
        }

        record[l][r]
    }
}
use std::i32;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(0, Solution::min_insertions(String::from("zzazz")));
        assert_eq!(0, Solution::min_insertions(String::from("zzaazz")));
        assert_eq!(2, Solution::min_insertions(String::from("mbadm")));
        assert_eq!(5, Solution::min_insertions(String::from("leetcode")));

        assert_eq!(
            25,
            Solution::min_insertions(String::from("tldjbqjdogipebqsohdypcxjqkrqltpgviqtqz"))
        );
    }
}
