```

```

####  在rust中，二叉树为什么要用`Option<Rc<RefCell<TreeNode>>>`包裹？

在 Rust 中，二叉树为什么要使用 `Option<Rc<RefCell<TreeNode>>>` 这样的结构来包裹节点呢？这涉及到 Rust 的所有权系统和借用检查器的限制。

1. **`Option`**: `Option` 是 Rust 标准库中的一个枚举类型，它用于表示一个值可能存在，也可能不存在的情况。在二叉树中，一个节点的左子节点和右子节点可能为空，因此可以使用 `Option` 来表示这种可能性。
    
2. **`Rc`**: `Rc` 是 Rust 中的引用计数智能指针（Reference Counting），允许多个所有者共享数据。在二叉树中，多个节点可能同时引用同一个节点，因此使用 `Rc` 可以方便地实现节点之间的共享。
    
3. **`RefCell`**: `RefCell` 是 Rust 中的内部可变性容器，允许在不可变引用的情况下修改其内部值。在二叉树中，由于需要在遍历或修改节点时改变节点的值，因此需要使用 `RefCell` 来实现内部可变性。
    

综合考虑以上因素，使用 `Option<Rc<RefCell<TreeNode>>>` 来包裹二叉树节点是为了在 Rust 的所有权和借用规则下，方便地管理节点的所有权、共享性和内部可变性，从而实现对二叉树的有效操作和管理。

#### 先要做一个把leetcode的数组变成rust中的二叉树的工具。
按照水平顺序遍历的， 空的地方是null

LeetCode 序列化二叉树的格式 - 力扣（LeetCode） - 支持 
https://support.leetcode.cn/hc/kb/article/1567641/ 

```go
func (this *Codec) deserialize(data string) *TreeNode {
	if data == "X" {
		return nil
	}
	list := strings.Split(data, ",")
	Val, _ := strconv.Atoi(list[0])
	root := &TreeNode{Val: Val}
	q := []*TreeNode{root}
	cursor := 1

	for cursor < len(list) {
		node := q[0]
		q = q[1:]
		leftVal := list[cursor]
		rightVal := list[cursor+1]
		if leftVal != "X" {
			v, _ := strconv.Atoi(leftVal)
			leftNode := &TreeNode{Val: v}
			node.Left = leftNode
			q = append(q, leftNode)
		}
		if rightVal != "X" {
			v, _ := strconv.Atoi(rightVal)
			rightNode := &TreeNode{Val: v}
			node.Right = rightNode
			q = append(q, rightNode)
		}
		cursor += 2
	}
	return root
}

作者：笨猪爆破组
链接：https://leetcode.cn/problems/serialize-and-deserialize-binary-tree/solutions/290289/shou-hui-tu-jie-gei-chu-dfshe-bfsliang-chong-jie-f/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

```


### leetcode 的 playground的 把字符串变成树的代码
```python
def stringToTreeNode(input):
    input = input.strip()
    input = input[1:-1]
    if not input:
        return None

    inputValues = [s.strip() for s in input.split(',')]
    root = TreeNode(int(inputValues[0]))
    nodeQueue = [root]
    front = 0
    index = 1
    while index < len(inputValues):
        node = nodeQueue[front]
        front = front + 1

        item = inputValues[index]
        index = index + 1
        if item != "null":
            leftNumber = int(item)
            node.left = TreeNode(leftNumber)
            nodeQueue.append(node.left)

        if index >= len(inputValues):
            break

        item = inputValues[index]
        index = index + 1
        if item != "null":
            rightNumber = int(item)
            node.right = TreeNode(rightNumber)
            nodeQueue.append(node.right)
    return root
```

然后再做 二叉树 相关的 题目。 




BFS 

```go
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func levelOrder(root *TreeNode) [][]int {
    ret := make([][]int, 0)

    if root==nil {
        return ret 
    }

    queue := make([]*TreeNode, 0)
    queue = append(queue, root)

    aLevel := make([]int, 0)
    currentLevelRemain := 1
    nextLevelCounter := 0

    for len(queue)!=0 {
        node := queue[0]
        
        if node.Left!=nil {
            queue = append(queue, node.Left)
            nextLevelCounter ++
        }
        if node.Right!=nil {
            queue = append(queue, node.Right)
            nextLevelCounter ++
        }

        queue = append(queue[:0], queue[1:]...)
        currentLevelRemain -- 
        aLevel = append(aLevel, node.Val)

        if currentLevelRemain==0 {
            ret = append(ret, aLevel)

            aLevel = make([]int, 0)
            currentLevelRemain = nextLevelCounter
            nextLevelCounter = 0
        }

    }

    return ret 
}
```



写了一天的297题
```rust

```

