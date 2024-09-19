fn main() {
    println!("Hello, world!");

    // let a = String::from("abcd");
    // let b = String::from("cdabcdab");
    let a = String::from("abc");
    let b = String::from("cabcabca");
    println!("{:?}", Solution::repeated_string_match(a, b));
}

struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        if Self::str_str(a.clone(), b.clone()) != -1 {
            println!("OK");
            return 1;
        }

        // 然后: 尝试翻倍a
        let mut count = 1;

        let ori_a = a.clone();
        let mut a = a.clone();
        while true {
            a.push_str(&ori_a);
            count += 1;
            println!("{:?} {:?} ", a.clone(), b.clone());
            if Self::str_str(a.clone(), b.clone()) != -1 {
                return count;
            }

            if a.len() > b.len() + 100 {
                return -1;
            }
        }

        -1
    }

    pub fn str_str(a: String, b: String) -> i32 {
        let next = Self::get_next(&b);
        // println!("next: {:?}", next);

        let mut a_idx = 0;
        let mut b_idx = 0;
        let a = a.into_bytes();
        let b = b.into_bytes();

        while a_idx < a.len() && b_idx < b.len() {
            if a[a_idx] == b[b_idx] {
                a_idx += 1;
                b_idx += 1;
            } else if b_idx != 0 {
                b_idx = next[b_idx] as usize;
            } else {
                a_idx += 1;
            }
        }

        // println!("{} {}", a_idx, b_idx);

        if b_idx == b.len() {
            return a_idx as i32 - b.len() as i32;
        } else {
            return -1;
        }
    }
    pub fn get_next(b: &String) -> Vec<i32> {
        let mut next = vec![-1; b.len()];

        let bb = b.as_bytes();

        for i in 1..next.len() {
            let mut last_val = next[i - 1];

            while last_val >= 0 && bb[last_val as usize] != bb[i - 1] {
                last_val = next[last_val as usize];
            }

            next[i] = last_val + 1;
        }

        next
    }
}
