


78. 子集 - 力扣（LeetCode） 
https://leetcode.cn/problems/subsets/solutions/2059409/hui-su-bu-hui-xie-tao-lu-zai-ci-pythonja-8tkl/ 


排列问题:   
全排列   

```rust

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let mut used = vec![false; nums.len()];
        Self::find_for(&mut res, &nums, &mut vec![], &mut used);

        res
    }

    fn find_for(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, now: &mut Vec<i32>, used: &mut Vec<bool>) {
        if now.len() == nums.len() {
            res.push(now.to_vec());
            return;
        }

        for idx in 0..nums.len() {
            if !used[idx] {
                now.push(nums[idx]);
                used[idx] = true;
                Self::find_for(res, nums, now, used);
                used[idx] = false;
                now.pop();
            }
        }
    }

}

```




组合问题:   








一个数组, 划分成为若干片段, 使用回溯找出每一种划分办法.
```python
{{#include ../../py_src/rev_slice.py}}
```



例题: 
- [131. 分割回文串 - 力扣（LeetCode）](https://leetcode.cn/problems/palindrome-partitioning/description/?envType=study-plan-v2&envId=top-100-liked)