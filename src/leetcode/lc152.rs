impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut record = vec![(0, 0); nums.len()];

        let mut setted = vec![false; nums.len()];

        // 容易两极反转, 所以需要同时记录最大值和最小值
        Self::find(&nums, &mut record, &mut setted, nums.len() - 1);

        let mut m = i32::MIN;
        for i in &record {
            m = m.max(i.0);
        }

        // println!("{:?}", record);
        m
    }

    fn find(
        nums: &Vec<i32>,
        record: &mut Vec<(i32, i32)>,
        setted: &mut Vec<bool>,
        idx: usize,
    ) -> (i32, i32) {
        if setted[idx] {
            return record[idx];
        }

        if idx == 0 {
            setted[idx] = true;
            record[idx] = (nums[0], nums[0]);
            return record[idx];
        }

        // match nums[idx] >= 0 {
        //     true => {
        //         record[idx] = (
        //             nums[idx].max(nums[idx] * Self::find(nums, record, setted, idx - 1).0),
        //             nums[idx].min(nums[idx] * Self::find(nums, record, setted, idx).1),
        //         );
        //     }

        //     false => {
        //         record[idx] = (
        //             nums[idx].max(nums[idx] * Self::find(nums, record, setted, idx - 1).1),
        //             nums[idx].min(nums[idx] * Self::find(nums, record, setted, idx).0),
        //         );
        //     }
        // }

        let p1 = nums[idx];
        let p2 = nums[idx] * Self::find(nums, record, setted, idx - 1).0;
        let p3 = nums[idx] * Self::find(nums, record, setted, idx - 1).1;

        let a = p1.max(p2).max(p3);
        let b = p1.min(p2).min(p3);

        record[idx] = (a, b);

        setted[idx] = true;
        record[idx]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::leetcode::lc152::Solution;

    #[test]
    fn it_works() {
        assert_eq!(6, Solution::max_product([2, 3, -2, 4].to_vec()));

        assert_eq!(-2, Solution::max_product([-2].to_vec()));
    }
}
