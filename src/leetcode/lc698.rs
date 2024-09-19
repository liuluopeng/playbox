impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let nums = nums.iter().map(|&x| x as usize).collect::<Vec<usize>>();

        // 创建一些数字, 每个数字代表一种选择方式 .  每个数字需要nums.len个bit.
        let mut record: Vec<Option<bool>> = vec![None; 1 << nums.len()];

        // Self::show_select_detail(32);

        let sum: usize = nums.iter().sum();
        let target_each_group = sum / k as usize;
        
        if sum % k != 0 {
            return false; 
        }

        // 初始状态: 全是没选择的.
        let res = Self::find(&nums, &mut record, 0, 0, target_each_group, k);

        // println!("{:?}", record);

        res
    }

    pub fn find(
        nums: &Vec<usize>,
        record: &mut Vec<Option<bool>>,
        situa_idx_now: usize,     // 当前的一种选择情况
        accu_now: usize,          // 当前group累计
        target_each_group: usize, // 当前group达到target的时候清零
        remain_group: usize,
    ) -> bool {
        if remain_group == 0 {
            return true;
        }

        if let Some(res) = record[situa_idx_now] {
            res
        } else {
            let mut res = false;

            for k_idx in 0..nums.len() {
                let pick_k_idx = match situa_idx_now & (1 << k_idx) {
                    0 => false,
                    value if value == (1 << k_idx) => true,
                    _ => panic!("panic"),
                };
                if !pick_k_idx {
                    if accu_now + nums[k_idx] == target_each_group {
                        // 选择nums[k_idx]
                        let new_situation_idx = situa_idx_now ^ (1 << k_idx);
                        let my_try = Self::find(
                            nums,
                            record,
                            new_situation_idx,
                            0,
                            target_each_group,
                            remain_group - 1,
                        );

                        // seset?  not need
                        res |= my_try;
                    } else if accu_now + nums[k_idx] < target_each_group {
                        let new_situation_idx = situa_idx_now ^ (1 << k_idx);
                        let my_try = Self::find(
                            nums,
                            record,
                            new_situation_idx,
                            accu_now + nums[k_idx],
                            target_each_group,
                            remain_group,
                        );
                        res |= my_try;
                    } else {
                        continue;
                    }
                }
            }

            record[situa_idx_now] = Some(res);
            res
        }
    }

    fn show_select_detail(val: usize) {
        //  第几个  有没有选择

        let mut check_who = 1;

        let mut counter = 1;
        while check_who < usize::MAX {
            println!(
                "{:?}  {:?}  {:?}  {:?}",
                val,
                check_who,
                counter,
                val & check_who == check_who
            );
            check_who *= 2;
            counter += 1;
        }
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4)
        );
    }
}
