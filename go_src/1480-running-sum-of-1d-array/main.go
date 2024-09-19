package main

import "fmt"

func runningSum(nums []int) []int {
	for i := 1; i < len(nums); i++ {
		nums[i] += nums[i-1]
	}
	return nums
}

func main() {
	arr := []int{3, 1, 2, 10, 1}
	slice := arr[:]
	fmt.Println(runningSum(slice))
}
