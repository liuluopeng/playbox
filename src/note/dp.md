# dp
上次看到动态规划是一个拓扑排序. 

```
当题目允许往任意方向移动时，考察的往往就不是 DP 了，而是图论。

从本质上说，DP 问题是一类特殊的图论问题。

那为什么有一些 DP 题目简单修改条件后，就只能彻底转化为图论问题来解决了呢？

这是因为修改条件后，导致我们 DP 状态展开不再是一个拓扑序列，也就是我们的图不再是一个拓扑图。

换句话说，DP 题虽然都属于图论范畴。

但对于不是拓扑图的图论问题，我们无法使用 DP 求解。

而此类看似 DP，实则图论的问题，通常是最小生成树或者最短路问题。



作者：宫水三叶
链接：https://leetcode.cn/problems/path-with-minimum-effort/solutions/581482/fan-zheng-fa-zheng-ming-si-lu-de-he-fa-x-ohby/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
```

## 01背包问题  





## 技巧: 二进制枚举   



## 状态压缩 




































## DP 

动态规划常常适用于有重叠子问题和最优子结构性质的问题，并且记录所有子问题的结果，因此动态规划方法所耗时间往往远少于朴素解法。

动态规划有自底向上和自顶向下两种解决问题的方式。自顶向下即记忆化递归，自底向上就是递推。

使用动态规划解决的问题有个明显的特点，一旦一个子问题的求解得到结果，以后的计算过程就不会修改它，这样的特点叫做**无后效性**，求解问题的过程形成了一张有向无环图。动态规划只解决每个子问题一次，具有天然剪枝的功能，从而减少计算量。

几个问题的状态转移方程


[爬楼梯](https://leetcode-cn.com/problems/climbing-stairs/)

- dp[n] 表示到达高度为n的楼梯有多少种办法.
- dp[0] = 0  
- dp[1] = 1   
- 此后, dp[n] = dp[n-1] + dp[n-2]

[使用最小花费爬楼梯](https://leetcode-cn.com/problems/min-cost-climbing-stairs/)
```
You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
You can either start from the step with index 0, or the step with index 1.
Return the minimum cost to reach the top of the floor.

Input: cost = [1,100,1,1,1,100,1,1,100,1]
Output: 6
Explanation: Cheapest is: start on cost[0], and only step on 1s, skipping cost[3].
```
- cost: [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
- dp[n] 表示到达高度为n的楼梯后花费的代价,已经加上了本阶楼梯上的数值了
- dp[0] = cost[0]
- dp[1] = cost[1]
- dp[2] = min(dp[1], dp[0]) + cost[2] 
- 由此可见,dp[n] = min(dp[n-1], dp[n-2]) + cost[n]

## [打家劫舍](https://leetcode-cn.com/problems/house-robber/)
```
你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。

输入：[2,7,9,3,1]
输出：12
解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
偷窃到的最高金额 = 2 + 9 + 1 = 12 。
```

状态转移方程:
- dp[n]表示偷前n+1个房屋的最大利益.
- dp[0] = house[0]  只有一家,就偷它.
- dp[1] = max(house[0], house[1])   只有两家的时候,不能都偷,就偷最大的.
- dp[2] = max(dp[1], house[2])      如果dp[1]已经比house[2]大了,为了避免报警,完全可以放弃house[2],那么,dp[2]的值就是的dp[1].
- 由此, dp[n] = max(dp[n-1], house[n] + dp[n-2]) 


## [拆分整数使乘积最大](https://leetcode-cn.com/problems/integer-break/)
```
给定一个正整数 n，将其拆分为至少两个正整数的和，并使这些整数的乘积最大化。 返回你可以获得的最大乘积。

输入: 10
输出: 36
解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36
```

状态转移方程:
- 设置dp[n] 为长为n的拆分后乘起来的最好结果.
- 显然,dp[0] = 0
- 显然,dp[1] = 1 
- 显然,dp[2] = 1 * 1 = 1 
- dp[3] = 2 * 1 = 2   长为3有1、1、1和1、2两种分割方法.
- dp[4] = 2 * 2 = 4   
- 对于n的长度, 尝试若干次max(j*(i-j) , j*dp[i-j] ),从这些尝试里面取出最大值作为dp[n]

 
   ```python
    class Solution:
        def integerBreak(self, n: int) -> int:
            dp = [0] * (n + 1)
            for i in range(2, n + 1):
                for j in range(i):
                    dp[i] = max(dp[i], j * (i - j), j * dp[i - j])
            return dp[n] 
   ```
  