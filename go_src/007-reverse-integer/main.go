package main

import (
	"fmt"
	"math"
)

func reverse(x int) int {
	if x > math.MaxInt32 || x < math.MinInt32 {
		return 0
	}

	res := 0

	// 321 -> 123
	if x >= 0 {
		for ; x != 0; x /= 10 {
			ge := x % 10
			res = 10 * res
			res += ge
		}
	} else {
		x = -x
		for ; x != 0; x /= 10 {
			ge := x % 10
			res = 10 * res
			res += ge
		}
		res = -res
	}

	if res > 2147483647 || res < -2147483648 {
		return 0
	}

	return res
}

func main() {
	res := reverse(1534236469)
	fmt.Println(res)
}
