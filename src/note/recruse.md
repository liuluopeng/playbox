

【科普】递归反人类？_哔哩哔哩_bilibili   
https://www.bilibili.com/video/BV1yr421p7PN/?spm_id_from=333.999.0.0&vd_source=fbbc4e0ecf01a7c9e182a449fd54b44f   


评论中, 出现了关于递归的学习两种学习办法:   
不模拟:  
```
刚学递归时，我觉得我做的最错误的就是，花费了很大精力去模拟它  
```
模拟:
```   
和你相反。我觉得我做的最正确的就是，花费了很大精力去模拟它  
```



### 递归——————计算机中最重要的算法思想

把原来的问题，转化为更小的同一问题。

递归函数的两部分：
- 递归基： 最基本的问题,它需要手写解决办法。  
- 使用同名的函数，把问题规模减小。
  
函数调用的系统栈用来记住离解决问题还有多远，这个栈先逐渐变高，再逐渐变低，最后消失。执行过程每进入一个新的函数体，栈上就入栈一个函数， 所以，栈是先越来越高，在它最高的时候，问题终于被划分到最小了，可以执行递归基的处理了。栈顶的函数被执行完成，这个调用就结束了，栈就出去一个元素。子函数执行结束后，它在父函数中就变成了一个结果，运行过程也从子函数回到了父函数中，继续执行下去。  当栈变成空栈的时候，意味着这个问题也解决了。  
递归是函数调用的特殊情况，一个函数调用了和它同名的、代码中同一块的函数，人眼看上去会想象它执行起来套娃的样子，但是在被运行前，它自己都不知道它调用的其实就是它自己。然而，虽然每次算法一样，但由于参数变小了，所以每次相同的算法，解决的问题规模都不一样。  越接近递归基，问题的规模就越小。。

其实，如果知道问题的规模，可以用不同的函数解决吗？


```c
#include <stdio.h>

void hanoi(int n, char x, char y, char z);

// x起始柱子  y终点柱子  z中转柱子
void hanoi(int n, char x, char y, char z){
	if (n==0){
		// printf("I am out\n");
		return;
	} else {
		// 把n-1个圆盘放到中转柱子上
		hanoi(n-1, x, z, y);
		// 把剩下的一个圆盘从起始柱子放到终点柱子  这个圆盘是最大的
		printf("%c->%c ", x, y);
		// 再把n-1个圆盘从中转柱子放到终点柱子上
		hanoi(n-1, z, y, x);
	}
}

int main() {
	hanoi(6, 'x', 'y', 'z');
	return 0;
}
		


```
```golang
package main

import "fmt"

func hanio(n int) int {
	times := 0

	var _hanio func(n int, begin, end, transit rune)
	_hanio = func(n int, begin, end, transit rune) {
		if n == 0 {
			return
		}
		_hanio(n-1, begin, transit, end)
		fmt.Printf("%c->%c\t", begin, end)
		times ++ 
		_hanio(n-1, transit, end, begin)
	}

	_hanio(n, 'A', 'B', 'C')

	fmt.Println("共计移动次数", times)
	return times
}

func main() {
	n := 31
	hanio(n)
}


/*
lxq@PC:~/Desktop/dataStuctureAndAlgorithm/88-算法思想/递归$ go run hanio.go 
共计移动次数 4294967295
lxq@PC:~/Desktop/dataStuctureAndAlgorithm/88-算法思想/递归$ go run hanio.go 
共计移动次数 2147483647
*/
```


## 递归 recr
引言 hanio 汉诺塔问题 

递推式：   

$H(n)$是移动完$n$个盘子需要的次数。
- 第一步: 为了给最大的盘子腾出位置，需要把它上面的$n-1$个盘子移动到中转的柱子上，移动需要借目标柱子来暂时放置，需要$H(n-1)$次
- 第二步： 最大的盘子在起始柱子上，直接移动到目标柱子上，需要$1$步。
- 第三步： 将中转柱子上的$n-1$个柱子移动到目标柱子上，移动需要借助起始柱子来放置，需要$H(n-1)$步   
    
所以，递推式是：

$H(n)=H(n-1)  +  1  +  H(n-1)$

