package main

import "fmt"

func removeDuplicates(nums []int) int {
	j := 0
	for i := 1; i < len(nums); i++ {
		if nums[i] == nums[j] {
			continue
		} else {
			nums[j+1] = nums[i]
			j++
		}
	}

	return j+1
}

func main() {
	a := []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}
	aa := a[:]
	fmt.Println(removeDuplicates(aa))
	fmt.Println(aa)
}
