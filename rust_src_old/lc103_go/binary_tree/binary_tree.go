package binary_tree

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

// Serialize 把二叉树变成字符串
func Serialize(root *TreeNode) string {

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

func Deserialize(data string) *TreeNode {
	st := data[1 : len(data)-1]
	st = strings.ReplaceAll(st, " ", "")

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
