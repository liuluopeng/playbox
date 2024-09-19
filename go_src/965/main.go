package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isUnivalTree(root *TreeNode) bool {

	if root.Left == nil && root.Right == nil{
		return true
	}

	if root.Left != nil {
		if root.Val != root.Left.Val {
			return false
		} else {
			return true
		}
	}

	if root.Right != nil {
		if root.Val != root.Right.Val {
			return false
		} else {
			return true
		}
	}

	return isUnivalTree(root.Left) && isUnivalTree(root.Right)
}

// func dfs(root *TreeNode) {

// 	if root != nil {

// 		dfs(root.Left)

// 		dfs(root.Right)
// 	}
// }

func main() {

	t := &TreeNode{Val: 3}
	t.Left = &TreeNode{Val: 4}
	t.Right = &TreeNode{Val: 3}

	fmt.Println(isUnivalTree(t))
}
