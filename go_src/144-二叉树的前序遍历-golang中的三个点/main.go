package main

import "fmt"

//  Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal1(root *TreeNode) []int {
	arr := make([]int, 0)

	if root == nil {
		return nil
	}

	arr = append(arr, root.Val)
	preorderTraversal1(root.Left)
	preorderTraversal1(root.Right)

	return arr
}

func preorderTraversal2(root *TreeNode) []int {
	arr := make([]int, 0)

	if root == nil {
		return nil
	}

	arr = append(arr, root.Val)
	arr = append(arr, preorderTraversal2(root.Left)...)
	arr = append(arr, preorderTraversal2(root.Right)...)

	return arr
}

func main() {

	root := &TreeNode{Val: 1}

	root.Right = &TreeNode{Val: 2}
	root.Right.Right = &TreeNode{Val: 3}

	fmt.Println(preorderTraversal2(root))
}
