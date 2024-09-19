package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// Array2List 将slice arr 生成一个链表
func Array2List(arr []int) *ListNode {
	list := &ListNode{}
	head := list
	for _, v := range arr {
		list.Next = &ListNode{Val: v}
		list = list.Next
	}

	return head.Next
}

func PrintList(head *ListNode) {
	for head != nil {
		fmt.Printf("%d -> ", head.Val)
		head = head.Next
	}
	fmt.Println("nil")

}

// 1 -> 2 -> 3 -> 4 -> 5 -> 4 -> nil
// 1 -> 2 -> 3 -> 5 -> nil



// 没用假的头
func removeElements2(head *ListNode, val int) *ListNode {

	for head != nil && head.Val == val {
		if head.Val == val {
			head = head.Next
		}
	}

	myhead := head

	// 记住头结点的地址 一会儿要返回
	myfirst := myhead

	if myhead != nil {

		for myhead.Next != nil {
			if myhead.Next.Val == val {
				myhead.Next = myhead.Next.Next

			} else {
				myhead = myhead.Next
			}
		}
	}

	return myfirst
}


// 用假头
func removeElements(head *ListNode, val int) *ListNode {
	myhead := &ListNode{}
	// 接上去
	myhead.Next = head
	myreturn := myhead 

	for myhead.Next != nil{
		if myhead.Next.Val == val{
			// 标头没变，标头下一个元素被下下个后面的元素替换了
			myhead.Next = myhead.Next.Next
		} else {
			myhead = myhead.Next
		}
	}

	return myreturn.Next
}


func main() {
	arr := []int{1, 1, 3, 2, 1, 3, 4, 5}
	s := arr[:]

	list := Array2List(s)

	PrintList(list)

	PrintList(removeElements(list, 1))
}
