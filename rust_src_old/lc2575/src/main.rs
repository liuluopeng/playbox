struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut res = vec![0; word.len()];

        let chars: Vec<char> = word.chars().collect();
        let mut tmp = 0i64;
        for (i, c) in chars.iter().enumerate() {
            let new_num = c.to_digit(10).unwrap() as i64;
            tmp = (tmp * 10 + new_num) % m as i64;

            if tmp == 0 {
                res[i] = 1;
            } else {
                res[i] = 0;
            }
        }

        res
    }
}
fn main() {
    println!("Hello, world!");

    let word = String::from("529282143571");
    let m = 4;

    let word = String::from("86217457695827338571");
    let m = 8;

    println!("{:?}", Solution::divisibility_array(word, m));
}
