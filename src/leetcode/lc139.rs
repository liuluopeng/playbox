impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let ss: Vec<char> = s.chars().collect();

        let dict: Vec<Vec<char>> = word_dict
            .iter()
            .map(|word| word.chars().collect())
            .collect();

        let mut record = vec![None; ss.len() + 1];

        Self::find(&ss, &dict, 0, &mut record).unwrap()
    }

    fn find(
        ss: &Vec<char>,
        dict: &Vec<Vec<char>>,
        idx_ss: usize,
        record: &mut Vec<Option<bool>>,
    ) -> Option<bool> {
        if idx_ss == ss.len() {
            return Some(true);
        }

        if record[idx_ss].is_none() {
            let mut res = false;
            for k in 0..dict.len() {
                // [idx-1  idx      idx+word.len()]

                // 检查单词内是否相同.
                let mut same_word = true;

                if idx_ss + dict[k].len() > ss.len() {
                    continue;
                }

                for word_idx in 0..dict[k].len() {
                    if ss[idx_ss + word_idx] == dict[k][word_idx] {
                        continue;
                    } else {
                        same_word = false;
                        break;
                    }
                }
                if !same_word {
                    continue;
                } else {
                    let after = Self::find(ss, dict, idx_ss + dict[k].len(), record).unwrap();
                    res = res | after;
                }
            }

            record[idx_ss] = Some(res)
        }

        record[idx_ss]
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
            Solution::word_break(
                String::from("applepenapple"),
                vec!["apple", "pen"].iter().map(|w| w.to_string()).collect()
            )
        );

        assert_eq!(
            false,
            Solution::word_break(
                String::from("catsandog"),
                vec!["cats", "dog", "sand", "and", "cat"]
                    .iter()
                    .map(|w| w.to_string())
                    .collect()
            )
        );

        assert_eq!(
            true,
            Solution::word_break(
                String::from("bb"),
                vec!["a", "b", "bbb", "bbbb"]
                    .iter()
                    .map(|w| w.to_string())
                    .collect()
            )
        );

        assert_eq!(
            false,
            Solution::word_break(
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"),
                vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
                    .iter()
                    .map(|w| w.to_string())
                    .collect()
            )
        );
    }
}
