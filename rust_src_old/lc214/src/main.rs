fn main() {
    println!("Hello, world!");


    let s = "abcd".to_string();

    println!("{:?}", Solution::shortest_palindrome(s)   );
}

struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        // let mut next = vec![];

        let ss: Vec<char> = s.clone().chars().collect();

        let mut ss_rev = ss.clone();
        ss_rev.reverse();

        print!("{:?}  {:?}", ss, ss_rev);
        s.clone()
    }
}
