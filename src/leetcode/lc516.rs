impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        fn find(memo: &mut Vec<Vec<i32>>, l: usize, r: usize, chs: &Vec<char>) -> i32 {
            println!("l: {} r: {}", l, r);

            if l > r {
                memo[l][r] = 0;
                return 0;
            }
            if l == r {
                memo[l][r] = 1;
                return memo[l][r];
            }
            if l + 1 == r {
                if chs[l] == chs[r] {
                    memo[l][r] = 2;
                } else {
                    memo[l][r] = 1;
                }
                return memo[l][r];
            }

            if memo[l][r] == -1 {
                let left_char = chs[l];
                let right_char = chs[r];

                if left_char == right_char {
                    memo[l][r] = find(memo, l + 1, r - 1, chs) + 2;
                } else {
                    let maybe1 = find(memo, l, r - 1, chs);
                    let maybe2 = find(memo, l + 1, r, chs);
                    // let maybe3 = find(memo, l + 1, r - 1, chs);

                    // println!("maybe1 {} mabye2 {}", maybe1, maybe2);

                    memo[l][r] = maybe1.max(maybe2);
                }
            }

            memo[l][r]
        }

        let chs: Vec<char> = s.chars().collect();
        let mut memo = vec![vec![-1; chs.len()]; chs.len()];
        find(&mut memo, 0, chs.len() - 1, &chs);

        for l in &memo {
            println!("{:?}", l);
        }

        memo[0][chs.len() - 1]
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::longest_palindrome_subseq("bbbab".to_string()));
    }
}
