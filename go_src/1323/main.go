package main

import (
	"fmt"
	"math"
)

func maximum69Number(num int) int {

	res := num

	length := 0
	for ; res > 0; res /= 10 {
		_ = res % 10
		length++
	}

	res = num / int(math.Pow10(length-1))
	if res == 6 {
		return num + 3*int(math.Pow10(length-1))
	}

	changeFlag := false
	for i := 0; i < length-1; i++ {
		res *= 10
		if num/int(math.Pow10(length-i-2))%10 == 6 && changeFlag == false {
			res += 9
			changeFlag = true
		} else {
			res += (num / int(math.Pow10(length-i-2))) % 10
		}

	}

	return res
}

func main() {
	fmt.Println(maximum69Number(666))
}
