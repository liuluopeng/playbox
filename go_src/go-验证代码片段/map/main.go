package main

import "fmt"

func main() {
	a := []int{1, 1, 1, 2, 2, 2, 3, 3, 3}

	aa := a[:]

	m := make(map[int]int)

	for _, v := range aa {
		m[v]++
	}

	fmt.Println(m)
}
