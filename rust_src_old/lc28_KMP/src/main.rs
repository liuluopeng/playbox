use std::mem::needs_drop;

fn main() {
    println!("Hello, world!");

    let haystack = String::from("fffdadbutsad");
    let needle = String::from("sad");

    println!("{:?}", Solution::str_str(haystack, needle));
}

struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut res = 0;

        let next = Solution::get_next_array(&needle);
        println!("{:?}", next);

        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        let mut idx1 = 0;
        let mut idx2 = 0;

        while idx1 < haystack.len() && idx2 < needle.len() {
            // 最自然的分支是:
            if haystack[idx1] == needle[idx2] {
                idx1 += 1;
                idx2 += 1;
            } else if idx2 != 0 {
                // s2向前的分支:
                idx2 = next[idx2] as usize;
            } else if idx2 == 0 {
                // 找不到的分支:
                idx1 += 1;
            }
        }

        if idx2 == needle.len() {
            // success
            idx1 as i32 - idx2 as i32
        } else {
            -1
        }
    }
    pub fn get_next_array(needle: &String) -> Vec<i32> {
        let needle: Vec<char> = needle.chars().collect();
        // println!("{:?}", needle);

        let mut next = vec![];

        let size = needle.len();

        if size == 0 {
            return next;
        }

        next.push(-1);

        if size == 1 {
            return next;
        }

        next.push(0);

        for idx in 2..size {
            let mut flex_idx = idx - 1;

            while next[flex_idx] >= 0 && needle[next[flex_idx] as usize] != needle[idx - 1] {
                flex_idx = next[flex_idx as usize] as usize;
            }

            next.push(next[flex_idx] as i32 + 1);
        }

        next
    }
}
