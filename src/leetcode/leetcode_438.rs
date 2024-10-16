impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res = Vec::new();

        let ss: Vec<char> = s.chars().collect();
        let pp: Vec<char> = p.chars().collect();

        let mut p_freq = vec![0; 26];

        for &o in pp.iter() {
            p_freq[o as usize - 'a' as usize] += 1;
        }

        let mut s_freq = vec![0; 26];

        for idx in 0..ss.len() {
            s_freq[ss[idx] as usize - 'a' as usize] += 1;

            if idx >= pp.len() {
                s_freq[ss[idx - pp.len()] as usize - 'a' as usize] -= 1;
            }

            if p_freq == s_freq {
                res.push(idx as i32 - pp.len() as i32 + 1);
            }
        }

        res
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use crate::leetcode::leetcode_438::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![0, 6],
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string())
        )
    }
}
