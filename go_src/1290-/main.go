package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func getDecimalValue(head *ListNode) int {

	sum := head.Val

	for head.Next != nil {
		sum = sum << 1
		sum += head.Next.Val
		head = head.Next
	}

	return sum
}

func main() {
	l := ListNode{Val: 1}
	l.Next = &ListNode{Val: 1}
	l.Next.Next = &ListNode{Val: 1}

	fmt.Println(getDecimalValue(&l))
}
