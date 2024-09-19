use std::vec;



struct Solution;


impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut my_temp = vec![];
        Self::dfs(&nums, 0, &mut res, &mut my_temp);
        res 
    }

    pub fn dfs(nums: &Vec<i32>, current_index: usize,  res:  &mut Vec<Vec<i32>>, my_temp: &mut Vec<i32>) {
        if current_index == nums.len() {
            println!("{} 到了 ",current_index);
            // 添加结果
            res.push(my_temp.to_vec());
            return 
        }

        my_temp.push(nums[current_index]);
        Self::dfs(nums, current_index + 1, res, my_temp);

        my_temp.pop();
        Self::dfs(nums, current_index + 1, res, my_temp);
    }
}


fn main() {
    println!("Hello, world!");

    println!("{:?}",  Solution::subsets(vec![1,2,3]));
}
