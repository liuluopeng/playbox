package main

import (
	"fmt"
	"strconv"
	"strings"
)

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

func arr2string(arr []int) string {
	st := "["

	lastIndex := len(arr) - 1

	for i, v := range arr {
		if i != lastIndex {
			st += strconv.Itoa(v) + ", "
		} else {
			st += strconv.Itoa(v)
		}
	}

	st += "]"

	return st
}

func main() {
	st := "[1,4,3,2,5,2]"
	head := string2listNode(st)

	x := 3
	res := partition(head, x)
	fmt.Println(listNode2arr(res))
}

func partition(head *ListNode, x int) *ListNode {

	lessDummy := &ListNode{
		Val: -1,
	}
	lessCurrent := lessDummy 
	moreDummy := &ListNode{
		Val: -1,
	}
	moreCurrent := moreDummy 


	for head != nil {
		val := head.Val 
		var nextTurn *ListNode
		if val < x {
			// 添加到less的链表后面
			nextTurn = head.Next 
			head.Next = nil 
			lessCurrent.Next = head 
			lessCurrent = lessCurrent.Next
		} else {
			nextTurn = head.Next 
			head.Next = nil 
			moreCurrent.Next = head 
			moreCurrent = moreCurrent.Next
		}

		head = nextTurn  
	}

	// fmt.Println(lessCurrent)
	lessCurrent.Next = moreDummy.Next

	return lessDummy.Next
}
