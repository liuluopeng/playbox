
300. 最长递增子序列 - 力扣（LeetCode） 
https://leetcode.cn/problems/longest-increasing-subsequence/description/ 


```rust

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut memo = vec![1; nums.len()]; // memo[i]: 以nums[i]结尾的子序列长度的最长可能

        for i in 0..nums.len() {
            //

            for j in 0..i {
                if nums[j] < nums[i] {
                    memo[i] = memo[i].max(1 + memo[j]);
                }
            }
        }

        for i in memo {
            res = res.max(i);
        }

        res
    }
}
```


todo 
借助一个辅助数组. 

