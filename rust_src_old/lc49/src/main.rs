use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res = vec![];

        let mut hm = HashMap::new();

        for word in strs {
            // 排序word， 作为键
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let  sorted_word: String = chars.into_iter().collect();

            // println!("{:?}", sorted_word);
            // 如果不存在：插入空的列表
            let one_res = hm.entry(sorted_word.clone()).or_insert(vec![]);
            one_res.push(word);

        }

        for (k, v) in &hm {
            println!("{:?} {:?}", k, v,);
            res.push(v.to_vec());
        }

        res
    }
}

fn main() {
    let strs = vec![
        String::from("abc"),
        String::from("cba"),
        String::from("abcc"),
    ];
    println!("{:?}", Solution::group_anagrams(strs));
}
