


计算出来$a^n$
按顺序计算的复杂度:
$a a a a a a ...$  => $O(n)$

归并一些中间结果. 一个整数可以表示成2的一些次方相加的结果.   
$a aa aaaa aaaaa$ => $O(log(n))$




快速幂






题目:

50. Pow(x, n) - 力扣（LeetCode） 
https://leetcode.cn/problems/powx-n/description/ 

{{#playground ../leetcode/lc50.rs}}









前置:
位运算. 如何知道一个bit是1还是0.

