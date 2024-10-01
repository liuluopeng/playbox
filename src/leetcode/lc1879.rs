use std::i32;

impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut known = vec![vec![0; nums2.len() + 1]; 1 << nums2.len()];
        let mut setted = vec![vec![false; nums2.len() + 1]; 1 << nums2.len()];

        let res = Self::find(&nums1, &nums2, 0, 0, &mut known, &mut setted, &mut vec![]);

        res
    }

    // idx: nums[1]进行到的位置
    pub fn find(
        nums1: &Vec<i32>,
        nums2: &Vec<i32>,
        state: usize,
        idx: usize,

        known: &mut Vec<Vec<i32>>,
        setted: &mut Vec<Vec<bool>>,

        msg: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if setted[state][idx] == true {
            return known[state][idx];
        }

        if state == ((1 << nums2.len()) - 1) && idx == nums1.len() {
            // 所有数字选完了,
            // println!("所有数字选完了,idx: {} state:{}  msg:{:?}", idx, state, msg);
            // msg.clear();
            return 0;
        }
        let mut sum = i32::MAX;
        for k in 0..nums2.len() {
            if state & (1 << k) == 0 {
                // nums[2]

                sum = sum.min(
                    (nums1[idx] ^ nums2[k])
                        + Self::find(nums1, nums2, state ^ (1 << k), idx + 1, known, setted, msg),
                );

                // msg.push(vec![nums1[idx], nums2[k]]);
            }
        }
        known[state][idx] = sum;
        setted[state][idx] = true;
        known[state][idx]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
