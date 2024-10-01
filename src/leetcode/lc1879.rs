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
    use crate::leetcode::lc1879::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            8,
            Solution::minimum_xor_sum([1, 0, 3].to_vec(), [5, 3, 4].to_vec())
        );

        assert_eq!(
            15088819,
            Solution::minimum_xor_sum(
                [
                    65022, 4657711, 8572489, 3336640, 7744043, 8672352, 6861299, 5122697, 2857375,
                    7539481, 8907966, 3311170
                ]
                .to_vec(),
                [
                    6030101, 8828015, 59043, 6529065, 9719816, 7144904, 6799001, 5637315, 9805075,
                    1136584, 8266168, 4154565
                ]
                .to_vec()
            )
        );
    }
}
