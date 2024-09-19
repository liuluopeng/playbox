struct Solution;

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let mut res = -9223372036854775808;
        Self::find( &mut vec![], 1, 0, &nums, &mut res);
        res
    }

    pub fn find(turn: &mut Vec<i32>, mul: i64, start: usize, nums: &Vec<i32>, res: &mut i64) {

        if turn.len() != 0 {
            if mul as i64 > *res  {
                *res = mul as i64;
            }
        }

        for i in start..nums.len() {
            turn.push(nums[i]);
            Self::find(turn, mul * nums[i] as i64, i + 1, nums, res);
            turn.pop();
        }
    }
}

pub fn main() {
    // let nums = vec![3, -1, -5, 2, 5, -9];
    // let nums = vec![-4,-5,-4];
    let nums = vec![6,-3,-4,8,4,7,6,4,7,7,-3,-6,9];

    let a = Solution::max_strength(nums);
    println!("{}", a);
}
