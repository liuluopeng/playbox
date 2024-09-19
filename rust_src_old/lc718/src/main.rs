fn main() {
    println!("Hello, world!");

    let nums1 = vec![1, 2, 3, 2, 1];
    let nums2 = vec![3, 2, 1, 4, 7];

    println!("{:?}", Solution::find_length(nums1, nums2));
}

struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max_length = 0;

        let size1 = nums1.len();
        let size2 = nums2.len();

        // ------------------------------------------------------------------>x 绝对坐标系
        //             [1, 2, 3, 2, 1];                     nums1保持不动
        // [3, 2, 1, 4, 7];                                     nums2初始位置
        //                         [3, 2, 1, 4, 7];     nums2结束位置

        let nums1_left_abs_index = size2 - 1;
        let nums1_right_abs_index = size2 + size1 - 2;

        for time in 0..=size1 + size2 - 2 {
            let nums2_left_abs_index = time;
            let nums2_right_abs_index = time + size2 - 1;

            let (abs_start, abs_end) = Solution::get_cross(
                nums1_left_abs_index,
                nums1_right_abs_index,
                nums2_left_abs_index,
                nums2_right_abs_index,
            );

            let mut counter = 0;
            let mut max_one_turn = 0;
            for abs_idx in abs_start..=abs_end {
                // 获得两个数组中的数据.
                let comp1 = nums1[abs_idx - nums1_left_abs_index];
                let comp2 = nums2[abs_idx - nums2_left_abs_index];
                // println!(
                //     "abs index: {:?} nums1: {:?} nums2:  {:?}",
                //     abs_idx, comp1, comp2
                // );

                if comp1 == comp2 {
                    counter += 1;
                    max_one_turn = max_one_turn.max(counter);
                } else {
                    counter = 0;
                }
            }

            max_length = max_length.max(max_one_turn);
        }

        max_length
    }

    /// 求闭区间的交集
    pub fn get_cross(a_l: usize, a_r: usize, b_l: usize, b_r: usize) -> (usize, usize) {
        (a_l.max(b_l), a_r.min(b_r))
    }
}

// 食堂 子弹
