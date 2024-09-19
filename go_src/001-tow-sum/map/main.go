package main

import "fmt"

func twoSum(nums []int, target int) []int {
	// res := make([]int, 2)
	m := make(map[int]int)

	for i, v := range nums {
		if e, ok := m[target-v]; ok {
			// fmt.Println(target - v)
			return []int{i, e}
		}

		m[v] = i
	}

	return nil
}

func main() {
	nums := [...]int{3, 2, 4}
	snums := nums[:]

	target := 6
	res := twoSum(snums, target)
	fmt.Println(res)
}
