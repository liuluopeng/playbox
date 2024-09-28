impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cadi_now = nums[0];
        let mut hp = 1;

        for i in 1..nums.len() {
            if hp == 0 {
                cadi_now = nums[i];
                hp = 1;
                continue;
            }

            if cadi_now == nums[i] {
                hp += 1;
            } else {
                hp -= 1;
            }
        }

        let mut cnt_of_cadi = 0;
        for &v in nums.iter() {
            if v == cadi_now {
                cnt_of_cadi += 1;
            }
        }

        if cnt_of_cadi > nums.len() / 2 {
            return cadi_now;
        } else {
            return -1;
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc169::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            Solution::majority_element([2, 2, 1, 1, 1, 2, 2].to_vec())
        )
    }
}
