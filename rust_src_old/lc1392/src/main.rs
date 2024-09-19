fn main() {
    println!("Hello, world!");
    let s = String::from("ababab");

    println!("{:?}", Solution::longest_prefix(s));
}

struct Solution;

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        // KMP NEXT[]
        let chars: Vec<char> = s.chars().collect();

        let mut next: Vec<i32> = vec![];

        let size = chars.len();

        if size <= 1 {
            return String::new();
        }

        //       0   1   2   3   4   5   6
        //       a   b   a   b   a   b   A
        //       -1  0   0   1   2   3   4

        // size > 1
        next.push(-1);
        next.push(0);

        for i in 2..=size {
            // println!("{} {}", i, chars[i]);

            let mut iip1_cnt = next[i - 1]; // string[i-1]的next的数值

            while iip1_cnt >= 0 {
                if chars[iip1_cnt as usize] == chars[i - 1] {
                    break;
                } else {
                    iip1_cnt = next[iip1_cnt as usize];
                    continue;
                }
            }

            next.push(iip1_cnt + 1);
        }

        println!("next: {:?}", next);

        let mut res = String::new();

        let collec_size = next[size];
        for i in 0..collec_size {
            res.push(chars[i as usize]);
        }

        res
    }
}
