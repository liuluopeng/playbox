package main

import "fmt"

func factorial(n int) int {
	res := 1
	for i := 1; i <= n; i++ {
		res *= i
	}
	return res
}

func trailingZeroes2(n int) int {
	res := factorial(n)
	fmt.Println(res)

	count := 0

	for ; res%10 == 0; res /= 10 {
		count++
	}

	return count
}


// >>> 1 * 2 * 3 * 4 * 5
// 120
// >>> 1 * 2 * 3 * 4 * 5 * 6
// 720
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7
// 5040
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8
// 40320
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9


// 362880
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10
// 3628800
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11
// 39916800
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12
// 479001600
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13
// 6227020800
// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14
// 87178291200


// >>> 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15
// 1307674368000

// 每多一个5的倍数 尾巴就会多一个0
func trailingZeroes(n int) int {
	
	count := 0

	for n > 1 {
		count += n/5
		n /= 5
	}

	return count
}

func main() {

	fmt.Println(trailingZeroes(15))
}
