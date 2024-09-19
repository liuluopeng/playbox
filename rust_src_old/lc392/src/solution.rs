pub struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let ss: Vec<char> = s.chars().collect();
        let tt: Vec<char> = t.chars().collect();

        let mut s_index = 0;
        let mut t_index = 0;

        while t_index < tt.len() && s_index < ss.len() {
            if ss[s_index] != tt[t_index] {
                t_index += 1;
            } else {
                // ss[s_index] == tt[t_index]
                s_index += 1;
                t_index += 1;
            }
        }

        s_index == ss.len() 
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        let s = String::from("axc");
        let t = String::from("ahbgdcd");
        assert_eq!(Solution::is_subsequence(s, t), false);

        let s = String::from("abc");
        let t = String::from("ahbgdcd");
        assert_eq!(Solution::is_subsequence(s, t), true);
    }
}
