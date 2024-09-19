pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut count = 0;

        // []
        let mut prime_checker = vec![true; n as usize];

        for i in 2..prime_checker.len() {
            let mut index = i;
            while index + i < prime_checker.len() {
                index += i;
                prime_checker[index] = false;
            }
        }

        for i in 2..prime_checker.len() {
            if prime_checker[i] {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(41537, Solution::count_primes(499979));
        assert_eq!(4, Solution::count_primes(10));
        assert_eq!(0, Solution::count_primes(0));
        assert_eq!(0, Solution::count_primes(1));
    }
}
