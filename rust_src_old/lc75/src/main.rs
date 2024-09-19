
struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut wait_small_index = 0usize;
        let comp = 1;
        for i in 0..nums.len() {
            if nums[i] < comp {
                nums.swap(i, wait_small_index);
                wait_small_index += 1;
            }
        }

        let mut wait_small_index = 0usize;
        let comp = 2;
        for i in 0..nums.len() {
            if nums[i] < comp {
                nums.swap(i, wait_small_index);
                wait_small_index += 1;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut arr = vec![0,2,2,1,1,0];
    Solution::sort_colors(&mut arr);
    println!("{:?}", &arr);
}
