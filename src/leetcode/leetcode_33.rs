impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for i in 0..nums.len() {
            if nums[i] == target {
                return i as i32;
            }
        }

        -1
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
