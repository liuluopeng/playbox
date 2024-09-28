use std::collections::HashMap;

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let target: Vec<char> = target.chars().collect();

        let stickers: Vec<Vec<char>> = stickers
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();

        let mut cnt_stickers = vec![];
        for sti in stickers.iter() {
            let mut m = HashMap::new();

            for &c in sti {
                *m.entry(c).or_insert(0) += 1;
            }

            cnt_stickers.push(m);
        }

        let mut target_cnt = HashMap::new();
        for c in &target {
            *target_cnt.entry(*c).or_insert(0) += 1;
        }
        23
    }
    // state: 001010101001  idx:代表:考虑idx的位置
    pub fn find(
        cnt_stickers: &Vec<HashMap<char, i32>>,
        state: usize,
        idx: usize,
        target: &Vec<char>,
        cnt_target: &mut HashMap<char, i32>,
    ) -> i32 {
        // for k in 0..target.len() {
        //     if state & (1 << k) == 0 {

        //         // 寻找一个贴纸, 贴纸里面有需要的target[k]这个字母
        //     }
        // }

        for ski in cnt_stickers {
            let mut new_state = state;

            for (k, v) in cnt_target.iter() {
                if ski.contains_key(k) {
                    // *cnt_target.entry(*k).or_default() -= v.min(ski.get(k).unwrap());
                }
            }
        }
        32
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc691::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::min_stickers(
                ["with", "example", "science"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                "thehat".to_string()
            )
        );
    }
}
