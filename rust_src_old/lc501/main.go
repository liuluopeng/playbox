/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findMode(root *TreeNode) []int {
	arr := inorderToArr(root)

	res := modeInArr(arr)

	return res
}

func inorderToArr(root *TreeNode) []int {
	arr := make([]int, 0)

	var dfs func(root *TreeNode)
	dfs = func(root *TreeNode) {
		if root != nil {
			dfs(root.Left)
			arr = append(arr, root.Val)
			dfs(root.Right)
		}
	}

	dfs(root)

	return arr
}

func modeInArr(arr []int) (res []int) {
	occMap := make(map[int]int)
	for _, num := range arr {
		occMap[num] += 1
	}

	fmt.Println(occMap)

	max_occ_time := -1
	for k, v := range occMap {
		fmt.Println(k)
		fmt.Println(v)
		if v > max_occ_time {
			max_occ_time = v
		}
	}

	res = make([]int, 0)

	for k, v := range occMap {
		if v == max_occ_time {
			res = append(res, k)
		}
	}

	return
}

func main() {

	var left TreeNode = TreeNode{
		Val:   3,
		Left:  nil,
		Right: nil,
	}

	var root TreeNode = TreeNode{
		Val:   3,
		Left:  &left,
		Right: nil,
	}

	fmt.Println(root)

	arr := findMode(&root)
	fmt.Println(arr)
}
