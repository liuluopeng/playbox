package main

import "fmt"

func getLeastNumbers(arr []int, k int) []int {
	for i := 0; i < len(arr); i++ {
		minIndex := i
		for j := i + 1; j < len(arr); j++ {
			if arr[j] < arr[minIndex] {
				minIndex = j
			}
		}
		arr[i], arr[minIndex] = arr[minIndex], arr[i]
	}

	return arr[:k]
}

func main() {
	a := []int{1, 3, 5, 9, 2, 0}
	arr := a[:]
	fmt.Println(getLeastNumbers(arr, 4))
}
