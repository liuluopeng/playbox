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

func deleteDuplicates2(head *ListNode) *ListNode {

	cur := head

	for cur.Next != nil {
		if cur.Val == cur.Next.Val {
			cur.Next = cur.Next.Next
		} else {
			cur = cur.Next
		}
	}

	return head
}

func deleteDuplicates(head *ListNode) *ListNode {

	// 如果List是空的
	if head == nil {
		return head
	}

	current := head.Val

	res := &ListNode{Val: current}
	myhead := res

	for head.Next != nil {
		if head.Next.Val == current {
			head = head.Next
		} else {
			res.Next = &ListNode{Val: head.Next.Val}
			res = res.Next
			current = head.Next.Val
			head = head.Next
		}
	}

	return myhead
}

func main() {
	a := []int{0, 0, 0, 1, 2, 3, 4, 4, 4, 5}
	b := a[:]

	c := array2list(b)
	printList(c)
	c = deleteDuplicates2(c)
	printList(c)
}
