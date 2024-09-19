package main

import "fmt"

func findMaxConsecutiveOnes(nums []int) int {
	count := 0
	maxCount := -2147483648
	for i := 0; i < len(nums); i++ {
		if nums[i] == 1 {
			count++
		} else {
			if count > maxCount {
				maxCount = count
			}
			count = 0
		}
	}
	return maxCount

}

func main() {

	a := []int{1, 1, 0, 1, 1, 1}

	aa := a[:]

	fmt.Println(findMaxConsecutiveOnes(aa))
}
