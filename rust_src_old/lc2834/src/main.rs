struct Solution;

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let n = n as u64;
        let target = target as u64;

        if n <= target / 2 {
            return ((1 + n) * n / 2 % 1000000007).try_into().unwrap();
        } else {
            // n >= target/2
            return (((1 + target / 2) * (target / 2) / 2
                + (target + target + n - target / 2 - 1) * (n - target / 2) / 2)
                % 1000000007)
                .try_into()
                .unwrap();
        }
    }
}

fn main() {
    println!("Hello, world!");

    let n = 45456;
    let target = 10867;

    println!("{}", Solution::minimum_possible_sum(n, target));
}
