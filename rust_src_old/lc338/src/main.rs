struct Solution;

impl Solution {

    pub fn count_bits_of_n(mut n:  i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            if n % 2 == 1 {
                count += 1;
            }
            n /= 2;
        }
        count
    }
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![];

        for i in 0..n+1 {
            res.push(Solution::count_bits_of_n(i))
        }

        res 
    }
}

fn main() {

    let res = Solution::count_bits(5);
    println!("{:?}", res);
}
