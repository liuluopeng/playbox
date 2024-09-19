impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut res = 0;

        let mut record = vec![vec![0; 5]; n as usize + 1];
        res += Self::find(n as usize, 0, &mut record);
        res += Self::find(n as usize, 1, &mut record);
        res += Self::find(n as usize, 2, &mut record);
        res += Self::find(n as usize, 3, &mut record);
        res += Self::find(n as usize, 4, &mut record);

        (res % 1000000007) as i32
    }

    /// len_now: 考虑长度是len_now的片段的结果.
    /// end_with_idx: 在len_now这个片段的最后一个元素[a e i o u]的某一个的index
    pub fn find(len_now: usize, end_with_idx: usize, record: &mut Vec<Vec<usize>>) -> usize {
        if len_now == 0 {
            return 0;
        }
        if len_now == 1 {
            return 1; // 所有可能的字符串分别是："a", "e", "i" , "o" 和 "u"。
        }

        let mut res = 0;
        /*
            0 每个元音 'a' 后面都只能跟着 'e'
            1 每个元音 'e' 后面只能跟着 'a' 或者是 'i'
            2 每个元音 'i' 后面 不能 再跟着另一个 'i'
            3 每个元音 'o' 后面只能跟着 'i' 或者是 'u'
            4 每个元音 'u' 后面只能跟着 'a'
        */
        if record[len_now][end_with_idx] == 0 {
            match end_with_idx {
                0 => {
                    // a 可以跟在那些字母的后面:
                    res += Self::find(len_now - 1, 1, record); // ea
                    res += Self::find(len_now - 1, 2, record); // ia
                    res += Self::find(len_now - 1, 4, record); // ua
                }
                1 => {
                    // e
                    res += Self::find(len_now - 1, 0, record); // ae
                    res += Self::find(len_now - 1, 2, record); // ie
                }
                2 => {
                    // i
                    res += Self::find(len_now - 1, 1, record); // e i
                    res += Self::find(len_now - 1, 3, record); // o i
                }
                3 => {
                    // ***o
                    res += Self::find(len_now - 1, 2, record); // i o
                }
                4 => {
                    // ...u
                    res += Self::find(len_now - 1, 2, record); // i u
                    res += Self::find(len_now - 1, 3, record); // o u
                }
                _ => {}
            }

            res %= 1000000007;
            record[len_now][end_with_idx] = res
        }

        record[len_now][end_with_idx]
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(68, Solution::count_vowel_permutation(5));
        assert_eq!(18208803, Solution::count_vowel_permutation(144));
    }
}
