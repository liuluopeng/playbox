


可以 维护: 
- 最大值
- 最小值
- 累加和 

不可以维护:   
- 某段 范围 数字x 出现的次数









区间染色问题：
在给定的区间内多次进行染色,可以覆盖前面的颜色,问最终区间会被染成多少种颜色。`m`次操作后，在区间`[i,j]`可以看到多少种颜色？

| 操作             | 数组实现的复杂度 | 如果用线段树 |
| ---------------- | ---------------- | ------------ |
| 染色（更新区间） | $O(n)$           | $O(log(n))$  |
| 查询（查询区间） | $O(n)$           | $O(log(n))$  |
对一个区间，左孩子和右孩子是它的两半。

<!-- ![gZny7R.jpg](https://z3.ax1x.com/2021/05/01/gZny7R.jpg) -->



 线段树不一定是完全二叉树。
 线段树一定是平衡二叉树， 叶子结点的深度相同，或者深度差1层。 

 一个有h层(based 0)的完全二叉树的总计结点个数是 $2^h-1$ 个。每一层的结点差不多都是前面所有层的和。   
 
<!-- ![等比数列求和公式](https://bkimg.cdn.bcebos.com/formula/2b6ca1d7d9f1f83bd90f17da866fdc92.svg) -->

每一层结点个数有`1 2 4 8 16 32...`
如果是满二叉树，区间长度为$n$，它满足$n=2^k$ 。则叶子结点的那一层一共有$n$个，叶子层之上的所有结点的和也为$n$,需要 $2n$ 的空间  
如果很不巧，多出一个，$n=2^k+1$ 则需要$2n+2n=4n$的空间 
