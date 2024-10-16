impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];

        let ch_list: Vec<char> = s.chars().collect();

        Self::backtrace(&mut res, &mut vec![], 0, &ch_list);
        res
    }

    pub fn backtrace(
        res: &mut Vec<Vec<String>>,
        now: &mut Vec<Vec<char>>,
        start_idx: usize,
        ch_list: &Vec<char>,
    ) {
        if start_idx == ch_list.len() {
            // println!("{:?}", now);
            // now.clear();

            let mut one_res = vec![];
            for c in 0..now.len() {
                let mut s = String::new();

                for kk in 0..now[c].len() {
                    s.push(now[c][kk]);
                }

                one_res.push(s);
            }

            res.push(one_res);
            return;
        }

        for k in start_idx..ch_list.len() {
            // now.push(ch_list[k]);

            // ch_list[start_idx ... k]
            let mut e = vec![];
            for j in start_idx..=k {
                e.push(ch_list[j]);
            }

            // check
            let mut pass = true;
            for i in 0..e.len() / 2 {
                if e[i] != e[e.len() - 1 - i] {
                    pass = false;
                    break;
                }
            }

            if !pass {
                continue;
            }

            now.push(e);
            Self::backtrace(res, now, k + 1, ch_list);
            now.pop();
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let s = Solution::partition("aab".to_string());
        println!("{:?}", s);
    }
}
