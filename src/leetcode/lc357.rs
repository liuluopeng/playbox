impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            let res = 1;
            return res;
        }

        // 1 :  10
        // 2 :  9 * 8
        // 3:   9 * 8 * 7

        let mut res = 10;

        let mut mul = 9;
        let mut k = 9;

        for i in 1..n {
            mul *= k;
            k -= 1;
            res += mul;
        }

        res
    }
}



use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(91, Solution::count_numbers_with_unique_digits(2));
    }
}
