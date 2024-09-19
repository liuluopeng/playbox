struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // 先找到在哪一行
        for row in &matrix {
            if row[0] <= target && target <= row[&matrix[0].len() - 1] {
                return Self::binary_search(&row, target);
            }
        }

        false
    }

    // nums是一个从小到大的有序数组
    pub fn binary_search(nums: &Vec<i32>, target: i32) -> bool {
        let mut left_index = 0;
        let mut right_index = nums.len() - 1;

        while left_index <= right_index {
            let mut mid_index = (left_index + right_index) / 2;

            // println!("left{} mid {} right{} ", left_index, mid_index, right_index);

            if nums[mid_index] == target {
                return true;
            } else if nums[mid_index] < target {
                left_index = mid_index + 1;
            } else {
                // target < nums[mid_index]
                right_index = mid_index - 1;
            }
        }

        false
    }
}
fn main() {
    println!("Hello, world!");

    let mat = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let res = Solution::search_matrix(mat, 7);
    println!("{}", res);
}
