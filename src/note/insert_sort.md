# 
### insert sort 
算法思想

打扑克的时候整理牌的顺序，从第二张牌开始，每一张牌往前看，当前面的一个位置正好适合放进这张牌的时候，把这张牌放进这个位置。

特点

插入排序一个很重要的特点就是， 如果元素大体上是有序的， 那么只需移动很少的元素，就能完成排序。

我的经验

**当不清楚边界坐标的时候，可以将特殊情况代入来验证。**

**想不清楚的时候，可以拿手指来模拟**

交换元素的问题:  

在排序中会大量需要交换元素这个操作，不同编程语言对swap()函数的实现不尽相同。

在Python、Golang等语言中，可以直接这样达到交换的目的:

`  a, b = b, a `

在C++中，命名空间std中提供了swap()函数可以直接使用。


在C语言中，可以自己实现一个swap()函数。


```c
void swap(int *a, int *b){
    int p = *a;
    *a = *b
    *b = p;
}

int swap(int* x, int* y){
	printf("x %p\n", x);
	printf("y %p\n", y);
	printf("*x %d\n",*x);  // *x => 1
	int tmp = *x;
	*x = *y;
	*y = tmp;
}


int main(){
	int a=1;
	int b=2;
	printf("&a %p, &b %p\n",&a, &b);
	swap(&a,&b);
	printf("a:%d b:%d\n",a,b);
}
```
