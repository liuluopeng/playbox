package main

import (
	"fmt"
	"strconv"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func binaryTreePaths(root *TreeNode) []string {

	res := make([]string, 0)

	type fp func(node *TreeNode, path string)
	var f fp
	f = func(node *TreeNode, path string) {

		if node != nil {

			path += strconv.Itoa(node.Val)
			if node.Left != nil || node.Right != nil {
				path += "->"
				f(node.Left, path)
				f(node.Right, path)
			} else {
				res = append(res, path)
				fmt.Println(path)
			}
		}
	}

	f(root, "")

	return res
}

func main() {

	root := &TreeNode{Val: 1}
	root.Left = &TreeNode{Val: 2}
	root.Right = &TreeNode{Val: 3}
	root.Left.Right = &TreeNode{Val: 5}

	fmt.Println(binaryTreePaths(root))
}
