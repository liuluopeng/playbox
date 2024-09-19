package main

import (
	"fmt"
	. "lc103/binary_tree"
)

func zigzagLevelOrder(root *TreeNode) [][]int {
	res := make([][]int, 0)

	if root == nil {
		return res
	}

	queue := []*TreeNode{root}
	left_direc := true
	for len(queue) != 0 {

		a_level := []int{}
		tmp_level := []*TreeNode{}
		for _, node := range queue {
			a_level = append(a_level, node.Val)
			if node.Left != nil {
				tmp_level = append(tmp_level, node.Left)

			}
			if node.Right != nil {
				tmp_level = append(tmp_level, node.Right)
			}
		}

		if left_direc {
			// 直接添加
			res = append(res, a_level)
		} else {
			for i := 0; i < len(a_level)/2; i++ {
				tmp := a_level[i]
				a_level[i] = a_level[len(a_level) - i - 1]
				a_level[len(a_level) - i - 1] = tmp 
			}
			res = append(res, a_level)
		}
		queue = tmp_level
		tmp_level = tmp_level[len(tmp_level):]
		a_level = a_level[len(a_level):]
		left_direc = !left_direc
	}

	return res
}

func main() {

	st1 := ""

	st1 = "[1,2,3,null,null,4,5]"
	st1 = "[3,9,20,null,null,15,7]"

	root := *Deserialize(st1)

	InOrder(&root)

	fmt.Println(Serialize(&root))

	fmt.Println(zigzagLevelOrder(&root))
}
