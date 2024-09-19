package main

import "fmt"

func findLucky(arr []int) int {
	m := make(map[int]int)

	for _, v := range arr {
		m[v]++
	}

	Lucky := make([]int, 0)
	for k := range m {
		if k == m[k] {
			Lucky = append(Lucky, k)
		}
	}

	fmt.Println(m, Lucky)

	max := 0
	if len(Lucky) == 0 {
		return -1
	} else {

		for _, v := range Lucky {
			if max < v {
				max = v
			}
		}
	}

	return max
}

func main() {
	a := []int{2, 2, 3, 4}
	aa := a[:]
	fmt.Println(findLucky(aa))
}
