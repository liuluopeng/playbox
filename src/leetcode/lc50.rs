use crate::solution::Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res = 1.0;
        let mut now_pow_idx = x;

        let n_usize = n.abs();

        for bit_idx in 0..32 {
            if (1 << bit_idx) & n_usize == 1 << bit_idx {
                res *= now_pow_idx;
            }

            now_pow_idx *= now_pow_idx;
        }

        if n < 0 {
            res = 1.0 / res;
        }

        res
    }
}
#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        assert_eq!(9.26100, Solution::my_pow(2.1, 3));
    }
}
