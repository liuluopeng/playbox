class Solution {
public:
    int numberOfSteps (int num) {
        int count = 0;
        while(num!=0){
        num % 2 ? num -= 1 : num/=2;
        count ++;
        } 
        return count;
    }
   
};


// HailStone序列
// 目前HailStone序列还未被证明是否有穷，所以它未必是一个算法。

// * HailStone序列
// * n=1时，返回1；
// * n>1时且为偶数时，{n} ∪ {n/2}
// * n>1时且为奇数时，{n} ∪ {3n + 1}