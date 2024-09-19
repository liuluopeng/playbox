package main

import "fmt"

func maxProfit2(prices []int) int {
	profit := 0

	for i := 0; i < len(prices); i++ {
		for j := i; j < len(prices); j++ {
			if prices[j]-prices[i] > profit {
				profit = prices[j] - prices[i]
			}
		}
	}

	return profit
}

func maxProfit(prices []int) int {

	if len(prices) == 0{
		return 0
	}

	profit := 0
	min := prices[0]

	for i := 1; i < len(prices); i++ {

		if prices[i] < min {
			min = prices[i]
		}

		if prices[i]-min > profit {
			profit = prices[i] - min
		}
	}

	return profit
}

func main() {
	a := []int{}
	aa := a[:]
	fmt.Println(maxProfit(aa))
}
