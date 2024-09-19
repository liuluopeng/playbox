use lc2761::solution::Solution;

fn main() {
    println!("Hello, world!");

    let n = 10;
    // let n = 1000000;
    // let n = 3;
    let n = 5;

    println!("{:?}", Solution::find_prime_pairs(n))
}
