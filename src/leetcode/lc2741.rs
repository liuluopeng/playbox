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
