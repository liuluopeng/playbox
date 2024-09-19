impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        fn find(
            // idx1 idx2:  [....idx1]  [.....idx2] 的结果
            text1: &Vec<char>,
            text2: &Vec<char>,
            idx1: usize,
            idx2: usize,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if idx1 == 0 || idx2 == 0 {
                memo[idx1][idx2] = 0;
                return 0;
            }

            if memo[idx1][idx2] == -1 {
                let maybe1 = find(text1, text2, idx1 - 1, idx2, memo);
                let maybe2 = find(text1, text2, idx1, idx2 - 1, memo);
                let maybe3 = find(text1, text2, idx1 - 1, idx2 - 1, memo);

                let mut maybe4 = 0;
                if text1[idx1 - 1] == text2[idx2 - 1] {
                    maybe4 = find(text1, text2, idx1 - 1, idx2 - 1, memo) + 1;
                }

                memo[idx1][idx2] = maybe1.max(maybe2).max(maybe3).max(maybe4);
            }

            memo[idx1][idx2]
        }

        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        let l1 = text1.len();
        let l2 = text2.len();
        let mut memo = vec![vec![-1; l2 + 1]; l1 + 1];

        find(&text1, &text2, text1.len(), text2.len(), &mut memo);

        for l in &memo {
            println!("line: {:?}", l);
        }

        
        memo[l1][l2]
    }
}
use std::mem;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence(String::from("abcde"), String::from("ace"))
        );
    }
}
