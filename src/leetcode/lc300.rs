impl Solution {
    // 最长递增子序列
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // 每次修改完ends之后, 数一数ends修改位置左侧有多少元素
        let mut ends = vec![];
        for idx in 0..nums.len() {
            if let Some(idx_ends) = Self::find_a_place_to_replace(&ends, nums[idx]) {
                ends[idx_ends] = nums[idx];
            } else {
                ends.push(nums[idx]);
            }

            // println!("ends: {:?}", ends);
        }

        ends.len() as i32
    }

    // ends是一个有序的增大的数组,  >= nums的下标
    fn find_a_place_to_replace(ends: &Vec<i32>, num: i32) -> Option<usize> {
        let mut left = 0;
        let mut right = ends.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if ends[mid] < num {
                left = mid + 1;
            } else {
                right = mid ;
            }
        }

        if left < ends.len() {
            Some(left)
        } else {
            None
        }
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        /*


         输入：nums = [10,9,2,5,3,7,101,18]
        输出：4
        解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。 */

        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));

        assert_eq!(
            1,
            Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7])
        );

        assert_eq!(3, Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]));
    }
}
