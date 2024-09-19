package main

import (
	"fmt"
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

func main() {

	st := "[40,20,60,10,30,50,70]"
	val := 25

	root := string2tree(st)

	root2 := insertIntoBST(root, val)
	st2 := tree2string(root2)
	fmt.Println(st2)
}


func insertIntoBST(root *TreeNode, val int) *TreeNode {

	// 递归基: 遇到的第一个空缺位置,这个空缺位置的父亲是一个叶子节点
	if root == nil {
		return &TreeNode{Val: val}
	}

	if val < root.Val {
		// 走 左侧 
		root.Left = insertIntoBST(root.Left, val) 
	} else if root.Val < val {
		root.Right = insertIntoBST(root.Right, val)
	}

	return root 	
}
