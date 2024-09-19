fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3, 4, 3];

    //    2  3  4 -1  -1
    //                     -1  -1
    // ans:
    // [2, 3, 4, -1, 4]
    println!("{:?}", Solution::next_greater_elements(nums));
}
struct Solution {}
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];

        let mut nums = nums.clone();
        nums.append(&mut nums.clone());
        println!("{:?}", nums);
        for (i, v) in nums.iter().enumerate().rev() {
            while stack.is_empty() == false && nums[i] >= nums[stack[stack.len() - 1]] {
                stack.pop();
            }
            if stack.is_empty() {
                res.push(-1);
            } else {
                res.push(nums[stack[stack.len() - 1]]);
            }

            stack.push(i);
            // if res.len() == nums.len()/2 {
            //     break;
            // }
        }
        res.reverse();

        res.drain(..res.len() / 2).collect()
    }
}
