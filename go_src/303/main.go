package main

import "fmt"

type NumArray struct {
	Index int
	Val   int
	Next  *NumArray
}

func Constructor(nums []int) NumArray {
	head := &NumArray{Index: 0, Val: nums[0]}
	c := head

	for i := 1; i < len(nums); i++ {
		head.Next = &NumArray{Index: i, Val: nums[i]}
		head = head.Next
	}
	return *c
}

func (this *NumArray) SumRange(i int, j int) int {


	sum := 0

	for this.Index != i {
		this = this.Next
	}

	for this.Index <= j  {
		sum += this.Val
		if this.Next != nil {
			this = this.Next
		} else {
			break
		}
	}

	return sum
}



func Print(head *NumArray){
	for head.Next != nil{
		fmt.Printf("%d %d ->    ", head.Index, head.Val)
		head = head.Next
	}

	fmt.Printf("%d %d\n", head.Index, head.Val)
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.SumRange(i,j);
 */

func main() {
	nums := []int{-2, 0, 3, -5, 2, -1}
	arr := Constructor(nums)

	Print(&arr)


	sum := arr.SumRange(0,5)
	fmt.Println(sum)
}
