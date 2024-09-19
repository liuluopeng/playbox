package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func printList(head *ListNode) {
	for head != nil {
		fmt.Printf("%d -> ", head.Val)
		head = head.Next
	}
	fmt.Println(" nil")
}

func array2list(arr []int) *ListNode {
	list := &ListNode{}
	head := list
	for _, v := range arr {
		list.Next = &ListNode{Val: v}
		list = list.Next
	}

	return head.Next
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	for headA.Val != headB.Val {
		if headA.Next == nil {

			headA = headB
		} else if headB.Next == nil {

			headB = headA
		} else {
			headA = headA.Next
			headB = headB.Next
		}

	}

	return headA
}

func main() {
	a := []int{4, 1, 8, 4, 5}
	b := a[:]

	a1 := []int{5, 6, 1, 8, 4, 5}
	b1 := a1[:]

	c := array2list(b)
	d := array2list(b1)



	e := getIntersectionNode(c,d)
	printList(e)
}
