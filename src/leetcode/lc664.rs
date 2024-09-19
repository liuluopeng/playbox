use std::i32;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::strange_printer(String::from("aaabbb")));

        assert_eq!(2, Solution::strange_printer(String::from("aba")));

        assert_eq!(3, Solution::strange_printer(String::from("abab")));

        assert_eq!(9, Solution::strange_printer(String::from("abcabcabcabc")));

        assert_eq!(
            19,
            Solution::strange_printer(String::from(
                "baacdddaaddaaaaccbddbcabdaabdbbcdcbbbacbddcabcaaa"
            ))
        );
    }
}

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let mut record = vec![vec![-1; s.len()]; s.len()];

        let ss: Vec<char> = s.chars().collect();

        Self::find(&ss, 0, ss.len() - 1, &mut record)
    }

    // find(ss[l...r]) : 打印ss[l..r]至少需要的次数
    fn find(ss: &Vec<char>, l: usize, r: usize, record: &mut Vec<Vec<i32>>) -> i32 {
        if l == r {
            return 1;
        }
        if l + 1 == r {
            if ss[l] == ss[r] {
                return 1;
            } else {
                return 2;
            }
        }

        if record[l][r] == -1 {
            let mut res = ss.len() as i32;
            if ss[l] == ss[r] {
                // 随便 选择 一侧

                res = Self::find(ss, l + 1, r, record);
            } else {
                // 分隔
                for k_idx in l..r {
                    let k_res =
                        Self::find(ss, l, k_idx, record) + Self::find(ss, k_idx + 1, r, record);

                    res = res.min(k_res);
                }
            }

            record[l][r] = res;
        }

        record[l][r]
    }
}
