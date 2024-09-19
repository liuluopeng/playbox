package main

import "fmt"

func twoSum(nums []int, target int) []int {
	res := make([]int, 2)
	for i, e1 := range nums {
		for j:=i+1;j<len(nums);j++{
			e2 := nums[j]
			if e1+e2 == target {
				res[0] = i
				res[1] = j
				fmt.Println(res)
				return res
			}
		}
	}

	return res
}

func main() {
	nums := [...]int{3,3}
	snums := nums[:]

	target := 6
	res := twoSum(snums, target)
	fmt.Println(res)
}
