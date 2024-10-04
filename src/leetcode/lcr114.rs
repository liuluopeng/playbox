impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph = vec![vec![false; 26]; 26];
        let mut in_list = vec![0; 26];

        let mut exist = vec![false; 26];

        for w in words.iter() {
            let ww: Vec<char> = w.chars().collect();
            for c in ww {
                exist[c as usize - 'a' as usize] = true;
            }
        }

        let mut cnt = 0;
        for &e in &exist {
            if e {
                cnt += 1;
            }
        }

        for ro in 0..words.len() {
            for co in 0..words.len() {
                if ro < co {
                    let a_word: Vec<char> = words[ro].chars().collect();

                    let b_word: Vec<char> = words[co].chars().collect();

                    let mut a_idx = 0;
                    let mut b_idx = 0;

                    while a_idx < a_word.len()
                        && b_idx < b_word.len()
                        && a_word[a_idx] == b_word[b_idx]
                    {
                        a_idx += 1;
                        b_idx += 1;
                    }

                    if a_idx < a_word.len()
                        && b_idx < b_word.len()
                        && a_word[a_idx] != b_word[b_idx]
                    {
                        // println!("diff: {} {}", a_word[a_idx], b_word[b_idx]);

                        if graph[a_word[a_idx] as usize - 'a' as usize]
                            [b_word[b_idx] as usize - 'a' as usize]
                            == false
                        {
                            graph[a_word[a_idx] as usize - 'a' as usize]
                                [b_word[b_idx] as usize - 'a' as usize] = true;
                            in_list[b_word[b_idx] as usize - 'a' as usize] += 1;
                        }
                    } else if a_idx < a_word.len()
                        && b_idx < b_word.len()
                        && a_word[a_idx] == b_word[b_idx]
                    {
                    }

                    if a_idx < a_word.len() && b_idx >= b_word.len() {
                        // println!("排序错误");
                        return String::new();
                    }
                }
            }
        }

        // 两两比较
        for k in 0..words.len() - 1 {}

        let mut queue: Vec<usize> = vec![];

        for k in 0..26 {
            if exist[k] && in_list[k] == 0 {
                queue.push(k);
            }
        }

        let mut res = String::new();
        while !queue.is_empty() {
            let k = queue.pop().unwrap();

            res.push((k + 'a' as usize) as u8 as char);

            // println!("{}", res);

            for i in 0..26 {
                if graph[k][i] == true {
                    in_list[i] -= 1;
                    if in_list[i] == 0 {
                        queue.push(i);
                    }
                }
            }
        }

        if res.len() != cnt {
            println!(
                "string:{}  string len:  {:?}   cnt {:?}",
                res,
                res.len(),
                cnt
            );
            return String::new();
        } else {
            return res;
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::leetcode::lcr114::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            "wertf".to_string(),
            Solution::alien_order(
                ["wrt", "wrf", "er", "ett", "rftt"]
                    .iter()
                    .map(|a| a.to_string())
                    .collect()
            )
        );

        assert_eq!(
            "z".to_string(),
            Solution::alien_order(["z", "z"].iter().map(|a| a.to_string()).collect())
        );

        assert_eq!(
            "".to_string(),
            Solution::alien_order(["abc", "ab"].iter().map(|a| a.to_string()).collect())
        );

        assert_eq!(
            "acbz".to_string(), // "cbaz"也行
            Solution::alien_order(
                ["ac", "ab", "zc", "zb"]
                    .iter()
                    .map(|a| a.to_string())
                    .collect()
            )
        );
    }
}
