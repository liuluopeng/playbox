package main

import (
	"fmt"
	"strconv"
	"strings"
)

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

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

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func insertGreatestCommonDivisors(head *ListNode) *ListNode {
	root := head

	for head.Next != nil {
		nextHead := head.Next

		newNode := ListNode{Val: gcd(max(head.Val, nextHead.Val), min(head.Val, nextHead.Val))}
		newNode.Next = nextHead
		head.Next = &newNode

		head = nextHead.Next
	}

	return root
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func main() {
	st := "[18, 6, 10, 3]"
	head := string2listNode(st)
	fmt.Println(listNode2arr(head))
}
