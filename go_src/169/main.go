package main

import "fmt"

func majorityElement2(nums []int) int {
	m := make(map[int]int)

	for _, v := range nums {
		m[v]++
	}

	// fmt.Println(m)

	theMax := 0
	for key := range m {
		if m[key] > m[theMax] {
			theMax = key
		}
	}

	return theMax
}

// 更快
func majorityElement(nums []int) int {
	major := nums[0]
	count := 1

	for i := 1; i < len(nums); i++ {

		if count == 0 {
			count = 1
			major = nums[i]
		} else if major == nums[i] {
			count++
		} else if major != nums[i] {
			count--
		}

	}

	return major
}

func main() {
	a := []int{2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
	b := a[:]
	c := majorityElement2(b)
	fmt.Println(c)
}
