use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hm = HashMap::new(); //
        let ss: Vec<char> = s.chars().collect();

        let mut res = 0; // 一直记着较大的结果。
        let mut r_index = 0usize;

        for i in 0..ss.len() {
            // 记录从ss[i]开始的不重复。
            if i > 0 {
                hm.remove(&ss[i - 1]);
            }

            while r_index < ss.len() && !hm.contains_key(&ss[r_index]) {
                println!("右指针移动  {}  {}", r_index, &ss[r_index]);
                hm.insert(ss[r_index], 1);
                r_index += 1;
            }

            res = Self::more(res, r_index as i32 - i as i32);
        }

        res
    }

    fn more(a: i32, b: i32) -> i32 {
        if a >= b {
            return a;
        }
        b
    }
}

fn main() {
    let s = String::from("pwwkew");
    println!("{:?}", Solution::length_of_longest_substring(s));
}
