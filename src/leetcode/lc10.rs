impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let mut record = vec![vec![false; p.len() + 1]; s.len() + 1];
        let mut setted = vec![vec![false; p.len() + 1]; s.len() + 1];
        Self::_is_match(0, 0, &s, &p, 0, &mut record, &mut setted)
    }

    //  [p_idx.......] [s_idx..................]
    fn _is_match(
        s_idx: usize,
        p_idx: usize,
        s: &Vec<char>,
        p: &Vec<char>,
        depth: usize,
        record: &mut Vec<Vec<bool>>,
        setted: &mut Vec<Vec<bool>>,
    ) -> bool {
        let space = " ".repeat(depth * 4);

        if setted[s_idx][p_idx] == true {
            return record[s_idx][p_idx];
        } else {
            match (s_idx >= s.len(), p_idx >= p.len()) {
                (true, true) => {
                    // s p 同时匹配完成
                    record[s_idx][p_idx] = true;
                }
                (true, false) => {
                    // s 匹配完成  p还有东西

                    // 看看[p p+1]的p+1是否是*, 如果是*, 就考虑当成匹配了0次char p跳过
                    if p_idx + 1 < p.len() && p[p_idx + 1] == '*' {
                        record[s_idx][p_idx] =
                            Self::_is_match(s_idx, p_idx + 2, s, p, depth + 1, record, setted);
                    } else {
                        record[s_idx][p_idx] = false;
                    }
                }

                (false, true) => {
                    // s还有东西   p匹配完成
                    record[s_idx][p_idx] = false;
                }

                (false, false) => {
                    // 看看[p  p+1]的p+1是否是*

                    if p_idx == p.len() - 1 && p_idx + 1 >= p.len() {
                        // p_idx <= p.len() - 1 p是最后一个
                        if s[s_idx] == p[p_idx] || p[p_idx] == '.' {
                            record[s_idx][p_idx] = Self::_is_match(
                                s_idx + 1,
                                p_idx + 1,
                                s,
                                p,
                                depth + 1,
                                record,
                                setted,
                            );
                        } else {
                            record[s_idx][p_idx] = false;
                        }
                    }

                    if p_idx + 1 < p.len() && p[p_idx + 1] == '*' {
                        // println!(
                        //     "{} s:{} p:{}",
                        //     space,
                        //     Self::slice(s, s_idx),
                        //     Self::slice(p, p_idx)
                        // );

                        // 匹配0次p char:  直接跳过p char.
                        let match_zero_time =
                            Self::_is_match(s_idx, p_idx + 2, s, p, depth + 1, record, setted);

                        // 匹配若干p char: 需要s char 和 p char 一样 才行
                        let mut match_some_time = false;
                        if s[s_idx] == p[p_idx] || p[p_idx] == '.' {
                            match_some_time =
                                Self::_is_match(s_idx + 1, p_idx, s, p, depth + 1, record, setted);
                        } else {
                            match_some_time = false;
                        }

                        record[s_idx][p_idx] = match_zero_time || match_some_time;
                    } else {
                        //P+1不是*, 好办, 严格匹配p char , s char
                        if s[s_idx] == p[p_idx] || p[p_idx] == '.' {
                            record[s_idx][p_idx] = Self::_is_match(
                                s_idx + 1,
                                p_idx + 1,
                                s,
                                p,
                                depth + 1,
                                record,
                                setted,
                            );
                        } else {
                            record[s_idx][p_idx] = false;
                        }
                    }
                }
            }
        }

        setted[s_idx][p_idx] = true;
        return record[s_idx][p_idx];
    }

    fn slice(chs: &Vec<char>, idx: usize) -> String {
        let mut s = String::new();

        for i in 0..=idx {
            s.push(chs[i]);
        }

        s
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_match(String::from("aab"), String::from("c*a*b"))
        );

        assert_eq!(
            false,
            Solution::is_match(String::from("ssippi"), String::from("s*p*i"))
        );

        assert_eq!(
            true,
            Solution::is_match(String::from("aaaai"), String::from("a*i"))
        );

        assert_eq!(
            true,
            Solution::is_match(String::from("aaaa"), String::from("a*"))
        );

        assert_eq!(
            false,
            Solution::is_match(String::from("aa"), String::from("a"))
        );

        assert_eq!(
            true,
            Solution::is_match(String::from("ab"), String::from(".*"))
        );

        assert_eq!(
            true,
            Solution::is_match(String::from("mississippi"), String::from("mis*is*ip*."))
        );

        assert_eq!(
            false,
            Solution::is_match(
                String::from("aaaaaaaaaaaaaaaaaaab"),
                String::from("a*a*a*a*a*a*a*a*a*a*")
            )
        );
    }
}
