
struct Solution;



impl Solution {
    // [1...n]  总共有n+1个
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut p = vec![];  // p[i]==true: i存在了。
        for i in 0..nums.len() + 2 {  // [0不用  1 2 3 ... n] len=n+1
            p.push(false);
        }


        for i  in nums {
            if p[i as usize] == true {
                return i
            }
            p[i as usize] = true;
        }

        0
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![3,1,3,4,2];

    println!("{:?}", Solution::find_duplicate(nums));

}
