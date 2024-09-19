fn main() {
    println!("Hello, world!");

    let words = vec![String::from("c"), String::from("c"), String::from("c")];

    let mut wf: WordFilter = WordFilter::new(words);
    let res = wf.f("a".to_string(), "p".to_string());

    println!("{:?}", res)
}

use std::collections::HashMap;

struct WordFilter {
    f_res: HashMap<Vec<String>, String>, // 前缀 后缀   所查找单词的 最大index
    pos_msg: HashMap<String, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut f_res: HashMap<Vec<String>, String> = HashMap::new();
        let mut pos_msg: HashMap<String, usize> = HashMap::new();

        for (i, word) in words.iter().enumerate() {
            if !pos_msg.contains_key(word) {
                // 列出所有的前缀\后缀组合
                let pre_list = Self::get_all_prefix(word);
                let suff_list = Self::get_all_suff(word);

                for pre in pre_list {
                    for suff in &suff_list {
                        f_res.insert(vec![pre.clone(), suff.to_string()], word.to_string());
                    }
                }
            }

            pos_msg.insert(word.to_string(), i);

            // println!("{:?}", word);
        }

        println!("{:?}", pos_msg);
        println!("{:?}", f_res);

        WordFilter {
            f_res: f_res,
            pos_msg: pos_msg,
        }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let word = self.f_res.get(&vec![pref, suff]);

        if word.is_some() {
            *self.pos_msg.get(word.unwrap()).unwrap() as i32
        } else {
            -1
        }
    }

    fn get_all_prefix(word: &String) -> Vec<String> {
        let mut res = vec![];

        for i in 0..word.len() {
            let pre = &word[0..i + 1];
            res.push(pre.to_string())
        }

        return res;
    }

    fn get_all_suff(word: &String) -> Vec<String> {
        let mut res = vec![];

        for i in 0..word.len() {
            let pre = &word[i..];
            res.push(pre.to_string())
        }

        return res;
    }
}
