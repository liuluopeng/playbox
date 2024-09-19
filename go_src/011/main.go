package main

import "fmt"

func bigger(x, y int) int {
	if x >= y {
		return x
	} else {
		return y
	}
}

func smaller(x, y int) int {
	if x <= y {
		return x
	} else {
		return y
	}
}

func maxArea(height []int) int {
	maxWater := 0

	left := 0
	right := len(height) - 1

	for left < right {

		maxWater = bigger(maxWater, (right-left)*smaller(height[left], height[right]))

		if height[left] > height[right] {
			right--
		} else {
			left++
		}
	}

	return maxWater
}

func main() {
	a := []int{1, 8, 6, 2, 5, 4, 8, 3, 7}
	b := a[:]
	fmt.Println(maxArea(b))
}
