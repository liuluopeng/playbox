# heap
## Heap


```
# 二叉树数组表示方式

## 用户:
数组中表示完全二叉树, 请你分别给出以0开头 和  以1开头计算的公式.

## ChatGPT:
对于表示完全二叉树的数组，以0开头和以1开头计算的公式如下：

1. 以0开头计算的公式：
   - 如果某个节点的索引为 \(i\)，则其左子节点的索引为 \(2i + 1\)，右子节点的索引为 \(2i + 2\)。
   - 如果某个节点的索引为 \(i\)，则其父节点的索引为 \(\lfloor \frac{i-1}{2} \rfloor\)。

2. 以1开头计算的公式：
   - 如果某个节点的索引为 \(i\)，则其左子节点的索引为 \(2i\)，右子节点的索引为 \(2i + 1\)。
   - 如果某个节点的索引为 \(i\)，则其父节点的索引为 \(\lfloor \frac{i}{2} \rfloor\)。
```

```go
type maxHeap struct {
    data []int 
    size int  
}

func (heap *maxHeap) insert(elem int) {
    heap.data[heap.size+1] = elem 
    heap.size ++ 

    i := heap.size 
    for i>1 && heap.data[i/2] < heap.data[i] {
        heap.data[i], heap.data[i/2] = heap.data[i/2], heap.data[i]
        i /= 2
    }
}


func (heap *maxHeap) pop() int {
    res := heap.data[1]
    heap.data[1] = heap.data[heap.size]
    heap.size -- 

    i := 1 
    for i*2 <= heap.size {
        j := i*2 
        if j+1<heap.size && heap.data[j] < heap.data[j+1] {
            j = j + 1
        }
        if heap.data[j] < heap.data[i] {
            break 
        }
        heap.data[j], heap.data[i] = heap.data[i],heap.data[j]
        i = j 
    }

    return res 
}

func maximumScore(a int, b int, c int) int {
    myHeap := maxHeap{make([]int, 4), 0}

    myHeap.insert(a)
    myHeap.insert(b) 
    myHeap.insert(c) 

    counter := 0 

    for {
        piles0 := myHeap.pop()
        piles1 := myHeap.pop() 

        if piles1==0 {
            break 
        }

        myHeap.insert(piles0-1)
        myHeap.insert(piles1-1)
        counter ++ 
    }

    return counter 
}
```

