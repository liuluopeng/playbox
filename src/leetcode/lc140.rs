impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let ss: Vec<char> = s.chars().collect();

        let dict: Vec<Vec<char>> = word_dict
            .iter()
            .map(|word| word.chars().collect())
            .collect();

        let mut record = vec![None; ss.len() + 1];

        let mut accu = vec![];
        let mut glob_accu = vec![];

        // Self::find(&ss, &dict, 0, &mut record, &mut accu, &mut glob_accu).unwrap();
        Self::find(&ss, &dict, 0, &mut record, &mut accu, &mut glob_accu);

        let mut res = vec![];

        for k in glob_accu {
            res.push(k.join(" "));
        }

        res
    }

    fn find(
        ss: &Vec<char>,
        dict: &Vec<Vec<char>>,
        idx_ss: usize,
        record: &mut Vec<Option<bool>>,
        accumulat: &mut Vec<String>,
        glob_accumu: &mut Vec<Vec<String>>,
    ) {
        if idx_ss == ss.len() {
            // println!("find : {:?}", accumulat);

            glob_accumu.push(accumulat.to_vec());

            // return Some(true);
            return;
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
                    // 已经累计了一个单词
                    accumulat.push(dict[k].clone().into_iter().collect::<String>());

                    Self::find(
                        ss,
                        dict,
                        idx_ss + dict[k].len(),
                        record,
                        accumulat,
                        glob_accumu,
                    );

                    // 是不是需要回退?
                    accumulat.pop();
                    // for i in 0..dict[k].len() + 1 {
                    //     accumulat.pop();
                    // }

                    // res = res | after;
                }
            }

            // record[idx_ss] = Some(res)
        }

        // record[idx_ss]
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["cats and dog", "cat sand dog"]
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<String>>(),
            Solution::word_break(
                String::from("catsanddog"),
                vec!["cat", "cats", "and", "sand", "dog"]
                    .iter()
                    .map(|w| w.to_string())
                    .collect()
            )
        );
    }
}
