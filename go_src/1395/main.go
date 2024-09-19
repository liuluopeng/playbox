package main

import "fmt"

func numTeams(rating []int) int {
	count := 0
	for i, vi := range rating {
		for j, vj := range rating {
			for k, vk := range rating {
				if i < j && j < k {
					if vi > vj && vj > vk || vi < vj && vj < vk {
						count++
					}
				}
			}
		}
	}

	return count
}

func main() {
	a := []int{2, 5, 3, 4, 1}
	b := a[:]

	numTeams(b)
	fmt.Println(numTeams(b))
}
