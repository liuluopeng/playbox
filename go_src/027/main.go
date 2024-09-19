package main

import "fmt"

func removeElement(nums []int, val int) int {
	newLength := 0
	for _, v := range nums {
		if v != val {
			fmt.Println(v, val)
			nums[newLength] = v
			newLength++
		}
	}

	fmt.Println(nums)
	return newLength
}

func main() {
	a := []int{3, 2, 2, 3}
	aa := a[:]

	fmt.Println(removeElement(aa, 3))
}
