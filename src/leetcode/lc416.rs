impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for n in &nums {
            sum += *n;
        }

        if sum % 2 == 1 {
            return false;
        }

        let mut record = vec![vec![false; 200 * 100]; nums.len()];
        let mut setted = vec![vec![false; 200 * 100]; nums.len()];

        Self::find(0, 0, &mut record, &nums, sum / 2, &mut setted)
    }

    // find(len, sum) :  选idx位置/不选idx位置. 的 结果
    fn find(
        idx: usize,
        sum_now: i32,
        record: &mut Vec<Vec<bool>>,
        nums: &Vec<i32>,
        target: i32,
        setted: &mut Vec<Vec<bool>>,
    ) -> bool {
        if idx == nums.len() {
            return false;
        }

        if sum_now == target {
            return true;
        } else if sum_now > target {
            return false;
        }

        if setted[idx][sum_now as usize] == false {
            let maybe1 = Self::find(idx + 1, sum_now + nums[idx], record, nums, target, setted);
            let maybe2 = Self::find(idx + 1, sum_now, record, nums, target, setted);

            let res = maybe1 || maybe2;
            record[idx][sum_now as usize] = res;
            setted[idx][sum_now as usize] = true;
        }

        record[idx][sum_now as usize]
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            false,
            Solution::can_partition(vec![
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 99, 97
            ])
        );
    }
}
