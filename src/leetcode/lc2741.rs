impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let ww: Vec<char> = word.chars().collect();

        // 区间konwn[start][end]: [start..end]有多少aeiou
        let mut known = vec![vec![0; ww.len()]; ww.len()];

        let mut setted = vec![vec![false; ww.len()]; ww.len()];

        Self::dp(&ww, &mut known, &mut setted, 0, ww.len() - 1);


        let mut cnt = 0;
        for i in 0..known.len() {
            for j in 0..known[0].len() {
                if known[i][j] == k  {
                    cnt += 1;
                }
            }
        }

        cnt 
    }

    fn dp(
        ww: &Vec<char>,
        known: &mut Vec<Vec<i32>>,
        setted: &mut Vec<Vec<bool>>,
        start: usize,
        end: usize,
    ) -> i32 {
        if setted[start][end] == true {
            return known[start][end];
        }

        if start == end {
            if ww[start] == 'a'
                || ww[start] == 'e'
                || ww[start] == 'i'
                || ww[start] == 'o'
                || ww[start] == 'u'
            {
                known[start][end] = 1;
                setted[start][end] = true;
            }

            setted[start][end] = true;
            return known[start][end];
        }

        let mut res = 0;
        if ww[start] == 'a'
            || ww[start] == 'e'
            || ww[start] == 'i'
            || ww[start] == 'o'
            || ww[start] == 'u'
        {
            res = 1 + Self::dp(ww, known, setted, start + 1, end);
        } else {
            res = Self::dp(ww, known, setted, start + 1, end);
        }

        known[start][end] = res;
        setted[start][end] = true;

        known[start][end]
    }
}
impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut ch_list = vec![0];

        while ch_list.len() < k as usize {
            let temp = ch_list.clone();

            for i in 0..temp.len() {
                ch_list.push((temp[i] + 1) % 26)
            }
        }

        ((ch_list[k as usize] as u8) + 'a' as u8) as char
    }
}

impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let mut total = 0;

        let mut known = vec![vec![0; nums.len()]; 1 << nums.len()];

        let mut setted = vec![vec![false; nums.len()]; 1 << nums.len()];

        let res = Self::find(
            0,
            0,
            &nums,
            None,
            &mut vec![],
            &mut total,
            &mut known,
            &mut setted,
        );

        println!("res: {}  total: {}", res % 1000000007, total);
        (res % 1000000007) as i32
    }

    pub fn find(
        state: usize,
        idx_now: usize,
        nums: &Vec<i32>,
        last_idx: Option<usize>,
        path_debug: &mut Vec<i32>,
        total: &mut usize,
        known: &mut Vec<Vec<usize>>,
        setted: &mut Vec<Vec<bool>>,
    ) -> usize {
        if state == ((1 << nums.len()) - 1) {
            // return 1;

            // println!("{:?} {:?}", state, path_debug);
            *total += 1;
            return 1;
        }

        if let Some(idx) = last_idx {
            if setted[state][idx] == true {
                return known[state][idx];
            }
        }

        let mut res = 0;

        for k in 0..nums.len() {
            if state & (1 << k) == 0 {
                if let Some(last) = last_idx {
                    if (nums[last] % nums[k] == 0 || nums[k] % nums[last] == 0) {
                        // path_debug.push(nums[k]);
                        res += Self::find(
                            state ^ (1 << k),
                            idx_now,
                            nums,
                            Some(k),
                            path_debug,
                            total,
                            known,
                            setted,
                        );

                        // path_debug.pop();
                    }
                } else {
                    // last is None
                    // path_debug.push(nums[k]);
                    res += Self::find(
                        state ^ (1 << k),
                        idx_now,
                        nums,
                        Some(k),
                        path_debug,
                        total,
                        known,
                        setted,
                    );
                    // path_debug.pop();
                    // return 1;
                }
            }
        }

        if let Some(last_idx) = last_idx {
            known[state][last_idx] = res % 1000000007;
            setted[state][last_idx] = true;
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc2741::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::special_perm([2, 3, 6].to_vec()));
        assert_eq!(
            178290591,
            Solution::special_perm(
                [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192].to_vec()
            )
        );

        assert_eq!(
            178290591,
            Solution::special_perm(
                [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8191].to_vec()
            )
        );
    }
}
