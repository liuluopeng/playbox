package main

import "fmt"

func sumZero(n int) []int {
	res := make([]int,0)

	for length := n / 2; length > 0; length-- {
		res = append(res, length)
		res = append(res, 0-length)
	}

	if n%2 == 1 {
		res = append(res, 0)
	}

	return res
}

func main() {
	fmt.Println(sumZero(5))
}
