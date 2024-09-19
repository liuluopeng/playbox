# int



补码.  



下面的两段代码是寻找一个数字 最右测 的1  的函数. 




```java
	// 得到i最右侧的1的状态
	// 其他位都是0
	public static int lowbit(int i) {
		return i & -i;
	}
```



rust的数组下标是usize类型. 没法用相反数:


```rust
fn find_rightmost_one(num: usize) -> usize {
    num & (!num + 1)
}
```

