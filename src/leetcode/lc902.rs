use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            20,
            Solution::at_most_n_given_digit_set(
                vec!["1", "3", "5", "7"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                100
            )
        );

        assert_eq!(
            29523,
            Solution::at_most_n_given_digit_set(
                vec!["1", "4", "9"].iter().map(|x| x.to_string()).collect(),
                1000000000
            )
        );

        assert_eq!(
            1,
            Solution::at_most_n_given_digit_set(
                vec!["7"].iter().map(|x| x.to_string()).collect(),
                8
            )
        );

        assert_eq!(
            35933218,
            Solution::at_most_n_given_digit_set(
                vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                74546987
            )
        );
    }
}

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n = n as usize;
        let digits: Vec<usize> = digits.iter().map(|x| x.parse().unwrap()).collect();

        let max_len = Self::get_len(n as usize);

        let mut dp = vec![0; max_len];
        let mut setted = vec![false; max_len];
        Self::digit_dp(&digits, 0, false, true, n, &mut dp, &mut setted) as i32
    }

    pub fn get_idx_digit(n: usize, idx: usize) -> usize {
        let mut n_by_digit = vec![];
        let mut n = n;
        while n > 0 {
            let digit = n % 10;
            n /= 10;
            n_by_digit.push(digit);
        }

        n_by_digit.reverse();

        n_by_digit[idx]
    }

    pub fn digit_dp(
        digits: &Vec<usize>,
        idx: usize,
        setted_before_idx: bool, // idx之前有数字吗
        limited_partial: bool,   // 前几个位置已经是n的一部分
        n: usize,
        dp: &mut Vec<usize>,
        setted: &mut Vec<bool>,
    ) -> usize {
        if idx == Self::get_len(n) {
            if setted_before_idx {
                return 1;
            } else {
                return 0;
            }
        }

        if setted[idx] == true && !limited_partial && setted_before_idx {
            return dp[idx];
        }

        let mut res = 0;

        if setted_before_idx {
        } else {
            // 此时: 已经不可能是n的一部分.

            // if idx + 1 < Self::get_len(n) {
            res += Self::digit_dp(digits, idx + 1, false, false, n, dp, setted);
            // }
        }

        let mut curr_digit = Self::get_idx_digit(n, idx);
        if !limited_partial {
            curr_digit = 9;
        }

        // println!("curr digit in n : {}  idx: {} ", curr_digit, idx);

        for &k in digits {
            let mut partial = false;

            if k > curr_digit {
                break;
            }
            if k == curr_digit {
                partial = true;
            }

            // if idx + 1 < Self::get_len(n) {
            res += Self::digit_dp(
                digits,
                idx + 1,
                true,
                limited_partial && partial,
                n,
                dp,
                setted,
            );
            // }
        }

        if !limited_partial && setted_before_idx {
            dp[idx] = res;
            setted[idx] = true;
        }
        res
    }

    pub fn get_len(a: usize) -> usize {
        let mut res = 0;

        let mut a = a;
        while a > 0 {
            a /= 10;
            res += 1;
        }

        res
    }

    // 回溯, 超时, 给出了具体的数字.
    pub fn find_recru(
        digits: &Vec<usize>,
        now: &mut Vec<usize>,
        max_len: usize,
        res: &mut Vec<Vec<usize>>,
        n: usize,
        res_num: &mut usize,
    ) {
        if now.len() == max_len {
            return;
        }

        if now.len() < max_len {
            // println!("{:?} a result", Self::gen_num(now));

            if now.len() > 0 && Self::gen_num(now) <= n {
                // res.push(now.to_vec());
                *res_num += 1;
            }
        }

        for idx in 0..digits.len() {
            now.push(digits[idx]);

            Self::find_recru(digits, now, max_len, res, n, res_num);
            now.pop();
        }
    }

    fn gen_num(now: &Vec<usize>) -> usize {
        let mut res = 0;

        let mut mul = 1;
        for idx in (0..now.len()).rev() {
            res += now[idx] * mul;
            mul *= 10;
        }
        res
    }
}
