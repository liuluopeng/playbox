fn main() {
    println!("Hello, world!");

    let st = String::from("001011");

    println!("{:?}", Solution::min_operations(st));
}

struct Solution;

impl Solution {
    pub fn abs(i: i32) -> i32 {
        if i >= 0 {
            return i;
        }
        -i
    }

    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut res = vec![];

        let chs: Vec<char> = boxes.chars().collect();
        for i in 0..chs.len() {
            let mut sum = 0;
            for (j, c) in chs.iter().enumerate() {
                if *c == '1' && j != i {
                    sum += Self::abs(j as i32 - res.len() as i32)
                }
            }

            res.push(sum);
        }

        res
    }
}
