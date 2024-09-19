package main

import (
	"fmt"
	"math"
)

// 题解
func findMaxAverage(nums []int, k int) float64 {

	max, sum := math.MinInt32, 0

	for i := 0; i < k; i++ {
		sum += nums[i]
	}

	max = sum

	for i := 1; i <= len(nums)-k; i++ {
		sum = sum - nums[i-1] + nums[i+k-1]
		if sum > max {
			max = sum
		}
	}

	return float64(max) / float64(k)
}

// 我的风格 杂乱
func findMaxAverage3(nums []int, k int) float64 {

	if k == 1 {
		res := -2147483648
		for _, v := range nums {
			if v > res {
				res = v
			}
		}
		return float64(res)
	}

	lastsum := 0

	head := nums[0]

	for i := k - 1; i >= 0; i-- {
		lastsum += nums[i]
	}

	MaxSum := lastsum
	fmt.Println(lastsum, MaxSum)

	for i := k; i < len(nums); i++ {

		newhead := nums[i-k+1]
		newtail := nums[i]

		fmt.Println(newhead, newtail)

		oneSum := lastsum - head + newtail

		head = newhead

		fmt.Println(oneSum, "Onesun")
		if oneSum > MaxSum {
			MaxSum = oneSum
		}
		lastsum = oneSum
	}

	fmt.Println(MaxSum)
	return float64(MaxSum) / float64(k)
}

// Time out
func findMaxAverage2(nums []int, k int) float64 {

	sum := -2147483648

	for i := k - 1; i < len(nums); i++ {
		oneSum := 0
		for j := i; j >= i-k+1; j-- {
			fmt.Println(nums[j])
			oneSum += nums[j]
		}

		fmt.Println()
		if oneSum > sum {
			sum = oneSum
		}
	}

	return float64(sum) / float64(k)
}

func main() {
	a := []int{0,1,1,3,3}
	aa := a[:]
	fmt.Println(findMaxAverage(aa, 4))
}
