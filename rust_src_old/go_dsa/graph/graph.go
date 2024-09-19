package graph

import (
	"fmt"
	"sort"
)

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Neighbors []*Node
 * }
 */

type Node struct {
	Val       int
	Neighbors []*Node
}

func (node *Node) DepthFirst() {

	visited := make(map[int]bool)

	var dfs func(node *Node)
	dfs = func(node *Node) {

		if node != nil {
			visited[node.Val] = true
			fmt.Println(*node)
			for _, n := range node.Neighbors {
				if _, ok := visited[n.Val]; ok {

				} else {

					dfs(n)
				}
			}
		}
	}

	dfs(node)
}

func LoadFromArr(arr [][]int) *Node {
	// start with one

	nodeList := make([]*Node, 0)

	for index := range arr {
		// fmt.Println(index, adjList)

		newNode := &Node{
			Val: index + 1,
		}

		nodeList = append(nodeList, newNode)
	}

	for index, adjList := range arr {
		node := nodeList[index]
		for _, n := range adjList {
			node.Neighbors = append(node.Neighbors, nodeList[n-1])
		}

	}

	if len(nodeList) > 0 {
		return nodeList[0]
	}
	return nil
}

// 自定义类型
type Person struct {
	Name string
	Age  int
}

// Person 类型的切片
type ByAge []Person

// 实现 sort.Interface 接口的 Len 方法
func (p ByAge) Len() int {
	return len(p)
}

// 实现 sort.Interface 接口的 Less 方法
func (p ByAge) Less(i, j int) bool {
	return p[i].Age < p[j].Age
}

// 实现 sort.Interface 接口的 Swap 方法
func (p ByAge) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func cloneGraph(node *Node) *Node {

	visited := make(map[int][]*Node)

	copyed := make([]*Node, 0)

	var dfs func(node *Node)
	dfs = func(node *Node) {
		if node != nil {
			visited[node.Val] = node.Neighbors

			newNode := &Node{
				Val: node.Val,
			}
			copyed = append(copyed, newNode)
			// insert in a right place :
			// for i, v := range copyed {
			// 	if node.Val > v.Val {
			// 		copyed = append(copyed[:i], append([]*Node{newNode}, copyed[i:]...)...)
			// 	} else if node.Val < v.Val {

			// 	}
			// }

			for _, nei := range node.Neighbors {
				if _, ok := visited[nei.Val]; ok {

				} else {
					dfs(nei)
				}
			}
		}
	}

	// add node:
	dfs(node)

	customSort := func(a, b int) bool {
		return copyed[a].Val < copyed[b].Val
	}
	sort.Slice(copyed, customSort)

	// add edge
	for k, v := range visited {
		node := copyed[k-1] // not certain yet

		for _, n := range v {
			node.Neighbors = append(node.Neighbors, copyed[n.Val-1])
		}
	}

	if len(copyed) > 0 {
		return copyed[0]
	}

	return nil
}
