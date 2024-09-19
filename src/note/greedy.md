

【算法讲解090【必备】贪心经典题目专题2】 https://www.bilibili.com/video/BV1Rp4y1973N/?share_source=copy_web&vd_source=5a2d7ecacd3ba507f3c18fde1192c687





## 例题 343. 整数拆分  

快速

1.   整数拆分 - 力扣（LeetCode） 
https://leetcode.cn/problems/integer-break/description/ 

1.   整数拆分 - LeetCode Wiki 
https://doocs.github.io/leetcode/lc/343/#_3 


使用 `均值不等式` 的证明: 
343. 整数拆分 - 力扣（LeetCode） 
https://leetcode.cn/problems/integer-break/solutions/29098/343-zheng-shu-chai-fen-tan-xin-by-jyd/ 





```rust
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n < 4 {
            return n - 1;
        }
        match n % 3 {
            0 => return (3 as i32).pow((n / 3) as u32),
            1 => return (3 as i32).pow((n / 3 - 1) as u32) * 4,
            _ => return (3 as i32).pow((n / 3) as u32) * 2,
        }
    }
}
```


## 例题  P1090 [NOIP2004 提高组] 合并果子 / [USACO06NOV] Fence Repair G - 洛谷 | 计算机科学教育新生态 
https://www.luogu.com.cn/problem/P1090 


哈夫曼编码. 

{{#playground  ../luogu/P1090.rs}}