// use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc68::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            ["This    is    an", "example  of text", "justification.  "]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            Solution::full_justify(
                [
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                16
            )
        );

        assert_eq!(
            ["What   must   be", "acknowledgment  ", "shall be        "]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            Solution::full_justify(
                ["What", "must", "be", "acknowledgment", "shall", "be"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                16
            )
        );
    }
}
struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let max_width = max_width as usize;
        let num_of_words = words.len();

        let mut idx = 0; // 用掉了多少单词

        let mut group = vec![]; // 每一行包含多少单词

        while idx < num_of_words {
            // 寻找这一行能包含多少个单词,
            let mut used_char = 0;
            let mut count_word_in_now_line = 0; // 这一行现在能包含多少单词
            let mut line = vec![];
            while used_char <= max_width {
                let mut atempt_put_len = 0;

                if count_word_in_now_line == 0 {
                } else {
                    // 不是第一个单词,需要至少一个添加空格
                    atempt_put_len += 1;
                }

                atempt_put_len += words[idx].len();

                match used_char + atempt_put_len <= max_width {
                    true => {
                        // 可以放下当前单词

                        count_word_in_now_line += 1;
                        used_char += atempt_put_len;
                        line.push(words[idx].clone());
                        idx += 1;
                        match idx {
                            x if x == num_of_words => {
                                break;
                            }
                            _ => {
                                // 后面还有单词
                                continue;
                            }
                        }
                    }
                    false => {
                        // 不能放下当前单词,  进入下一行
                        break;
                    }
                }
            }

            group.push(line);
        }

        println!("line number {:?}", group);

        if group.len() == 1 {
            let mut last_line = group[group.len() - 1].join(" ");
            while last_line.len() < max_width {
                last_line.push(' ');
            }
            res.push(last_line);
            return res;
        }

        // 然后 给每一行 分配空格.
        for idx_line in 0..group.len() - 1 {
            // 需要多少空格
            let total_space: usize =
                max_width - group[idx_line].iter().map(|s| s.len()).sum::<usize>();
            // println!("space: {:?}", total_space);

            if group[idx_line].len() == 1 {
                let mut new_line = group[idx_line][0].clone();
                while new_line.len() < max_width {
                    new_line.push(' ');
                }
                res.push(new_line);
                continue;
            }

            // 能否整除.而且除数至少是1.    不能就左边分配多.
            match total_space % (group[idx_line].len() - 1) {
                0 => {
                    let each_space_len = total_space / (group[idx_line].len() - 1);
                    let same_space = " ".repeat(each_space_len);

                    let new_line = group[idx_line].join(&same_space);
                    res.push(new_line);
                }
                _ => {
                    // 发牌算法

                    let mut remain_space = total_space;
                    let mut line_words = group[idx_line].clone();

                    let mut who = 0;
                    while remain_space > 0 {
                        if who == line_words.len() - 1 {
                            who = 0;
                        }

                        line_words[who].push(' ');
                        remain_space -= 1;
                        who += 1;
                    }

                    res.push(line_words.join(""));
                }
            }
        }

        // 最后一行左对齐.
        // let mut last_line = String::new();

        // for w in &group[group.len() - 1] {
        //     last_line.push_str(w);
        //     last_line.push(' ');
        // }

        let mut last_line = group[group.len() - 1].join(" ");
        while last_line.len() < max_width {
            last_line.push(' ');
        }
        res.push(last_line);

        res
    }
}
