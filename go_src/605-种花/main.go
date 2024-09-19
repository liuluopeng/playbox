package main

import "fmt"

func canPlaceFlowers(flowerbed []int, n int) bool {

	// 两边 0 ....... 0

	zero := make([]int, 1)
	zero[0] = 0
	flowerbed = append(flowerbed, 0)
	flowerbed = append(zero, flowerbed...)

	fmt.Println(flowerbed)

	// 还能种的花
	count := 0
	for i := 2; i < len(flowerbed); {
		if flowerbed[i] == 0 && flowerbed[i-1] == 0 && flowerbed[i-2] == 0 {
			count++
			i += 2
		} else {
			i++
		}
	}

	return count >= n
}

func main() {

	a := []int{0, 0, 1, 0, 0}
	aa := a[:]

	fmt.Println(canPlaceFlowers(aa, 1))
}
