
# 矩阵
## rust把leetcode的输入变成二维的向量
```rust

// 在rust中，给二维数组每行增加一个“vec”
fn conv(input: &str) -> Vec<Vec<i32>> {
    let input = input.trim_matches(|c| c == '[' || c == ']' || c == ',');

    let mut result: Vec<Vec<i32>> = Vec::new();
    for inner_vec_str in input.split("],[").map(|s| s.split(',').collect::<Vec<_>>()) {
        let inner_vec: Vec<i32> = inner_vec_str
            .iter()
            .filter_map(|&x| x.parse().ok())
            .collect();
        result.push(inner_vec);
    }
    result
}
```

# 链表

## 链表 反序列化 golang
```go
package main

type ListNode struct {
	Val  int
	Next *ListNode
}

// "[18,6,10,3]" ->  18->6->10->3
func string2listNode(st string) *ListNode {
	st = st[1 : len(st)-1]
	st = strings.ReplaceAll(st, " ", "")
	valList := strings.Split(st, ",")

	dummyHead := &ListNode{Val: 0}
	head := dummyHead
	for _, val := range valList {
		val, _ := strconv.Atoi(val)
		newNode := &ListNode{Val: val}
		head.Next = newNode
		head = head.Next
	}

	return dummyHead.Next
}

func listNode2arr(head *ListNode) []int {
	arr := []int{}
	for head != nil {
		// fmt.Println(head.Val)
		arr = append(arr, head.Val)
		head = head.Next
	}
	return arr
}

func arr2string(arr []int) string {
	st := "["

	lastIndex := len(arr) -  1

	for i, v := range arr {
		if i != lastIndex {
			st += strconv.Itoa(v) + ", "
		} else {
			st += strconv.Itoa(v)
		}
	}

	st += "]"

	return st
}

```
# 二叉树 序列化、反序列化


## rust
```rust
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}


pub fn count_node(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut counter = 0;

    fn find(root: &Option<Rc<RefCell<TreeNode>>>, counter: &mut i32) {
        match root {
            None => {}
            Some(node) => {
                let node = node.borrow();
                // println!("访问的节点是 {:?}", node.val);
                *counter += 1;

                find(&node.left, counter);
                find(&node.right, counter);
            }
        }
    }

    find(root, &mut counter);
    counter
}
// 把[1,None,2]变成字符串
pub fn nodelist2string(nodelist: Vec<Option<i32>>) -> String {
    let mut res = String::from("[");

    for i in 0..nodelist.len() {
        match nodelist[i] {
            None => res.push_str("null"),
            Some(val) => {
                res += &val.to_string();
            }
        }
        if i != nodelist.len() - 1 {
            res.push(',');
        }
    }

    res.push(']');
    res
}
fn tree2string(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut node_list: Vec<Option<i32>> = vec![];
    let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

    let mut last_index = count_node(&root); // 最后一个叶子节点的序号

    match root {
        None => {}
        Some(node) => {
            queue.push(Some(node));
        }
    }

    let mut index = 0;

    while !queue.is_empty() {
        let node = queue.remove(0);
        // println!("index: {:?}", index);
        match node {
            None => {
                node_list.push(None);
            }
            Some(node) => {
                node_list.push(Some(node.borrow().val));
                index += 1;

                queue.push(node.borrow().left.clone());
                queue.push(node.borrow().right.clone());

                if index == last_index {
                    break;
                }
            }
        }
    }

    nodelist2string(node_list)
}

fn string2tree(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let mut t_sstr = data.to_string();
    t_sstr = t_sstr[1..t_sstr.len() - 1].to_string();

    if t_sstr.len() == 0 {
        return None;
    }

    let node_list: Vec<String> = t_sstr.split(",").map(|s| s.to_string()).collect();

    //  可以算出深度  可以算出  父节点是谁。
    let mut tree_node_list = vec![];
    // 创建节点
    for (index, st) in node_list.iter().enumerate() {
        if *st != String::from("null") {
            let node = Rc::new(RefCell::new(TreeNode::new(st.parse().unwrap())));
            tree_node_list.push(node);
        } else {
            // 空的， 先放一些数字的0的节点占住index。
            let node = Rc::new(RefCell::new(TreeNode::new(550)));
            tree_node_list.push(node);
        }
    }

    // for n in &tree_node_list {
    //     println!("节点打印 {:?}", n);
    // }

    let head = &tree_node_list[0];

    let mut queue = vec![tree_node_list[0].clone()];
    let mut front = 0;
    let mut index = 1;

    while index < node_list.len() {
        let node = queue[front].clone();
        front += 1;

        // println!("当前节点 {:?}  现在的index {:?}", node.borrow().val, index);

        let str_item = node_list[index].clone();
        if str_item != "null".to_string() {
            // println!("构建关系 节点 {:?} 左子树 {:?}", node.borrow().val, tree_node_list[index].borrow().val);
            node.borrow_mut().left = Some(tree_node_list[index].clone());
            queue.push(tree_node_list[index].clone());
        }
        index += 1;

        if index == node_list.len() {
            break;
        }

        let str_item = node_list[index].clone();
        if str_item != "null".to_string() {
            // println!("构建关系 节点 {:?} 右子树 {:?}", node.borrow().val, tree_node_list[index].borrow().val);

            node.borrow_mut().right = Some(tree_node_list[index].clone());
            queue.push(tree_node_list[index].clone());
        }
        index += 1;
    }

    Some(head.clone())
}




use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

fn main() {

}
```

## go 

```go
package main

import (
	"strconv"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// InOrder 中序遍历
func InOrder(root *TreeNode) int {
	if root != nil {
		return InOrder(root.Left) + InOrder(root.Right) + 1
	} else {
		return 0
	}
}

func tree2string(root *TreeNode) string {

	type item struct {
		value *int
	}

	res := ""

	itemList := make([]item, 0)

	// 水平遍历。
	queue := make([]*TreeNode, 0)

	if root != nil {
		queue = append(queue, root)
	}

	size := InOrder(root)
	index := 0
	for len(queue) != 0 {
		node := queue[0]

		if index == size {
			break
		}
		queue = append(queue[:0], queue[1:]...)

		if node != nil {
			val := node.Val
			index++
			itemList = append(itemList, item{&val})

			queue = append(queue, node.Left)
			queue = append(queue, node.Right)
		} else {
			var nilVal item
			itemList = append(itemList, nilVal)
		}

	}

	for i, v := range itemList {
		if v.value == nil {
			res += "null"
		} else {
			res += strconv.Itoa(*v.value)
		}

		if i != len(itemList)-1 {
			res += ", "
		}
	}

	res = "[" + res
	res = res + "]"
	return res
}

func string2tree(data string) *TreeNode {
	st := data[1 : len(data)-1]

	nodeList := strings.Split(st, ",")

	if len(st) == 0 {
		return nil
	}

	rootVal, _ := strconv.Atoi(nodeList[0])
	root := TreeNode{
		Val:   rootVal,
		Left:  nil,
		Right: nil,
	}
	queue := []*TreeNode{&root}

	nextIndex := 1

	for nextIndex < len(nodeList) {

		node := queue[0]

		queue = append(queue[:0], queue[1:]...)

		leftVal := nodeList[nextIndex]

		if leftVal != "null" {
			val, _ := strconv.Atoi(leftVal)
			leftNode := TreeNode{
				Val: val,
			}
			node.Left = &leftNode

			queue = append(queue, node.Left)
		}

		nextIndex++
		if nextIndex == len(nodeList) {
			break
		}

		rightVal := nodeList[nextIndex]
		if rightVal != "null" {
			val, _ := strconv.Atoi(rightVal)
			rightNode := &TreeNode{
				Val: val,
			}
			node.Right = rightNode

			queue = append(queue, node.Right)
		}

		nextIndex++
	}

	return &root
}
```

## python
```python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
# Definition for a binary tree node.
from typing import Optional

def treeNodeToString(root):
    if not root:
        return "[]"
    output = ""
    queue = [root]
    current = 0
    while current != len(queue):
        node = queue[current]
        current = current + 1

        if not node:
            output += "null, "
            continue

        output += str(node.val) + ", "
        queue.append(node.left)
        queue.append(node.right)
    return "[" + output[:-2] + "]"

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


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
```