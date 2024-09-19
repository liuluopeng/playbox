impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let mut record = vec![vec![false; p.len() + 1]; s.len() + 1];

        let mut setted = vec![vec![false; p.len() + 1]; s.len() + 1];

        Self::find(&mut record, &mut setted, 0, 0, &s, &p);

        record[s.len()][p.len()]
    }

    /*

        '?' 可以匹配任何单个字符。
    '*' 可以匹配任意字符序列（包括空字符序列）。
         */

    fn find(
        record: &mut Vec<Vec<bool>>,
        setted: &mut Vec<Vec<bool>>,

        row: usize,
        col: usize,

        s: &Vec<char>,
        p: &Vec<char>,
    ) -> bool {
        if setted[row][col] == false {
            let mut res = false;

            match (row == s.len(), col == p.len()) {
                (true, true) => {
                    res = true;
                }
                (false, true) => {
                    // [......sss]s  [......]p
                    res = false;
                }
                (true, false) => {
                    if p[col] == '*' {
                        res = Self::find(record, setted, row, col + 1, s, p);
                    } else {
                        res = false;
                    }
                }
                (false, false) => {
                    if p[col] == '*' {
                        let maybe_use_0 = Self::find(record, setted, row, col + 1, s, p);

                        let mut maybe_use_some_time = false;
                        maybe_use_some_time = Self::find(record, setted, row + 1, col, s, p);

                        res = maybe_use_0 || maybe_use_some_time;
                    } else {
                        if s[row] == p[col] || p[col] == '?' {
                            res = Self::find(record, setted, row + 1, col + 1, s, p);
                        } else {
                            res = false;
                        }
                    }
                }
            }

            record[row][col] = res;
            setted[row][col] = true;
        }

        record[row][col]
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            false,
            Solution::is_match(String::from("aa"), String::from("a"))
        );

        assert_eq!(
            true,
            Solution::is_match(String::from("aa"), String::from("*"))
        );

        assert_eq!(
            false,
            Solution::is_match(String::from("cb"), String::from("?a"))
        );
    }
}
