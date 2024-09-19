struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let lower = s.to_lowercase();

        let ss = lower.as_bytes();

        let mut res = 0;
        for i in 0..ss.len() - 1 {
            println!("{}  {}", i, ss[i]);
            if ss[i] != ss[i + 1] {
                res += 1;
            }
        }

        res
    }
}

fn main() {
    println!("Hello, world!");

    // let solution = Solution;
    let s = "jjj".to_string();
    let res = Solution::count_key_changes(s);
    println!("{:?}", res);
}
