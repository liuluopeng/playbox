package main

import (
	"fmt"
	"math"
)

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	// 测出长度
	length := 0
	copy := x
	for ; copy != 0; copy /= 10 {
		_ = copy % 10
		length++
	}

	for i := 1; i < length; i++ {
		head := x / int(math.Pow10(length-i))
		headOfHead := head % 10
		tail := x % int(math.Pow10(i))
		tailOfTail := tail

		if tail < int(math.Pow10(i-1)) {
			tailOfTail = 0
		} else {
			tailOfTail = tail / int(math.Pow10(i-1))
		}

		if headOfHead != tailOfTail {
			return false
		}
	}

	return true
}

func main() {
	// isPalindrome(1000000021)
	fmt.Println(isPalindrome(100343001))
}
