use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::old_vec_2d_leetcode};

    #[test]
    fn it_works() {
        /*
        输入：n = 4, k = 9
        输出："2314"
         */

        assert_eq!("2314".to_string(), Solution::get_permutation(4, 9));
    }
}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;

        let nums: Vec<i32> = (1..=n as i32).collect();

        let mut used = vec![false; n];

        let mut res = String::new();

        Self::find(&mut vec![], &mut 0, k, &nums, &mut used, &mut res);

        res
    }

    fn find(
        path: &mut Vec<i32>,
        idx_: &mut usize,
        target_idx: usize,
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        res: &mut String,
    ) {
        if path.len() == nums.len() {
            // println!("{:?}", path);

            *idx_ = *idx_ + 1;

            if *idx_ == target_idx {
                // 找到了
                *res = path
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                return;
            }
        }

        for idx in 0..nums.len() {
            if !used[idx] {
                path.push(nums[idx]);
                used[idx] = true;

                Self::find(path, idx_, target_idx, nums, used, res);
                path.pop();
                used[idx] = false;
            }
        }
    }
}
