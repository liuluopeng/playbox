package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func printList(l *ListNode) {
	for l != nil {
		fmt.Printf("%d -> ", l.Val)
		l = l.Next
	}
	fmt.Printf("nil \n")
}

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {

	// 假头
	first := 0
	res := &ListNode{Val: first}
	head := res

	for l1 != nil || l2!= nil {

		if l1 == nil {

			res.Next = &ListNode{Val: l2.Val}
			l2 = l2.Next
			res = res.Next

		} else if l2 == nil {
			res.Next = &ListNode{Val: l1.Val}
			l1 = l1.Next
			res = res.Next

		} else if l1.Val > l2.Val {
			res.Next = &ListNode{Val: l2.Val}
			l2 = l2.Next
			res = res.Next
		} else {

			res.Next = &ListNode{Val: l1.Val}
			l1 = l1.Next
			res = res.Next
		}

	}

	return head.Next

}

func main() {
	l1 := &ListNode{Val: 1}
	l1.Next = &ListNode{Val: 3}
	l1.Next.Next = &ListNode{Val: 9}

	l2 := &ListNode{Val: 2}
	l2.Next = &ListNode{Val: 5}

	res := mergeTwoLists(l1, l2)

	printList(l1)
	printList(l2)
	printList(res)
}
