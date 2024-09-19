package main

import (
	"fmt"
	"strconv"
)

func thousandSeparator(n int) string {
	res := ""

	copy := n
	length := 0
	for copy > 0 {
		copy /= 10
		length++
	}

	for n > 999 {

		slice := n % 1000

		if slice < 100 && slice > 9 {
			res = "0" + strconv.Itoa(n%1000) + res
		} else if slice < 10 {
			res = "00" + strconv.Itoa(n%1000) + res
		} else {
			res = strconv.Itoa(n%1000) + res
		}

		res = "," + res

		n /= 1000

		length -= 3
	}

	res = strconv.Itoa(n) + res

	return res
}

func main() {
	fmt.Println(thousandSeparator(40))
}
