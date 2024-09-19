impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let mut used = vec![false; nums.len()];
        Self::find_for(&mut res, &nums, &mut vec![], &mut used);

        res
    }

    fn find_for(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, now: &mut Vec<i32>, used: &mut Vec<bool>) {
        if now.len() == nums.len() {
            res.push(now.to_vec());
            return;
        }

        for idx in 0..nums.len() {
            if !used[idx] {
                now.push(nums[idx]);
                used[idx] = true;
                Self::find_for(res, nums, now, used);
                used[idx] = false;
                now.pop();
            }
        }
    }


    fn find_each(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, idx_now:usize) {
        
    }

}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3];

        println!("{:?}", Solution::permute(nums));
    }
}
