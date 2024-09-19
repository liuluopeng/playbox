# golang



# 语法

## **package的引用** 

目录的样子： 
``` 
C:\COMMON\MY GO DSA 2022
└─dsa
    ├─binary_search
    ├─heap
    ├─linkedArray
    ├─my_sort
    └─tree
```
package包的引用格式：
```go
import (
	binarysearch "dsa/dsa/binary_search"
	mysort "dsa/dsa/my_sort"
	dsa "dsa/dsa"
	"fmt"
	"math/rand"
	"time"
)
```


## `for index, value := range nums {}`

- i 是索引,
- v 是数组nums[i]的值，可以省略
- 有时候写`for i := range nums`我会以为`i`是`value`。


# 写法
## 使用函数类型 在函数里面再定义一个函数
```go
func f(){
    // f_internal 是要定义的小函数
	var f_internal func()int
	f_internal = func() int {
		return 0
	}


	ret := f_internal()
	fmt.Println(ret)
}
```


## 使用随机数的写法
```go
import "math/rand"

func myFunc() {
    //将时间戳设置成种子数
    rand.Seed(time.Now().UnixNano())
    //生成10个0-99之间的随机数
    for i:=0;i<10;i++ {
        fmt.Println(rand.Intn(100))
    }
}
```


## 切片中删除元素
要从切片a中删除索引为index的元素，操作方法是a = append(a[:index], a[index+1:]...)

## 清空切片
http://www.randyfield.cn/post/2021-07-20-go-how-to-clear-slice/
如果仅仅是清空切片，后续继续append操作，博主更推荐切片表达式的方法[0:0] 

## 拷贝切片
Go语言的内置函数 `copy()` 可以将一个数组切片复制到另一个数组切片中，如果加入的两个数组切片不一样大，就会按照其中较小的那个数组切片的元素个数进行复制。

copy() 函数的使用格式如下：
`copy( destSlice, srcSlice []T) int`

其中 `srcSlice` 为数据来源切片，`destSlice` 为复制的目标（也就是将 `srcSlice` 复制到 `destSlice`），目标切片必须分配过空间且足够承载复制的元素个数，并且来源和目标的类型必须一致，`copy()`函数的返回值表示实际发生复制的元素个数。

*copy后， 底层数组已经变了*

## 利用`copy()`,可以实现切片中的元素集体移动。
```go 
arr1 := []int{0,1,2,3,4,5,6,7,8,9,10}
arr2 := []int{0,1,2,3,4,5,6,7,8,9,10}

slice1 := arr1[:]
fmt.Println("原来的切片 ", slice1)
copy(slice1[1:], slice1[0:])
fmt.Println("全体向右移动一格后的切片", slice1)

slice2 := arr2[:]
fmt.Println("原来的切片 " , slice2)
copy(slice2[:], slice2[1:])
fmt.Println("全体向左移动一格后的切片", slice2)

// 运行结果：
// 原来的切片               [0 1 2 3 4 5 6 7 8 9 10]
// 全体向右移动一格后的切片  [0 0 1 2 3 4 5 6 7 8 9]
// 原来的切片               [0 1 2 3 4 5 6 7 8 9 10]
// 全体向左移动一格后的切片  [1 2 3 4 5 6 7 8 9 10 10]

```



## `[]byte`类型 `string`类型 的转换

```golang
    // string to []byte
    s1 := "hello"
    b := []byte(s1)

    // []byte to string
    s2 := string(b)
```

# 注意的地方

### 在变化的slice中 使用 for range
剑指offer 替换空格   
```go
func replaceSpace(s string) string {
    sslice := []rune(s) 
    
    for i,char := range sslice {    // 此处sslice是固定长度的 是初始化的sslice 一旦i超出初始长度就会退出
        if char==' ' {
            sslice = append(sslice, ' ') 
            sslice = append(sslice, ' ')

            copy(sslice[i+2:], sslice[i:])

            sslice[i] = '%'
            sslice[i+1] = '2'
            sslice[i+2] = '0'
        }
    }

    return string(sslice)
}
```






### 使用标准库的排序函数

`sort.Slice`
`func Slice(slice interface{}, less func(i, j int) bool)`


