package main

import (
	"fmt"
)

func addDigits(num int) int {

	res := 0

	for {
		one := 0
		for num > 0 {
			one = num % 10
			res += one
			num /= 10
		}
		num = res
		if res < 10 {
			break
		} else {
			res = 0
			continue
		}
	}

	return res
}

func main() {
	fmt.Println(addDigits(0))
}
