package main

import "fmt"

func triangleNumber(nums []int) int {
	if len(nums) < 3 {
		return 0
	}

	sort(nums)
	fmt.Println(nums)
	count := 0

	for i := 0; i < len(nums)-2; i++ {
		for j := i + 1; j < len(nums)-1; j++ {
			maxThird := nums[i] + nums[j]

			l := j + 1
			r := len(nums) - 1

			for l < r {
				mid := (l + r + 1) / 2

				if nums[mid] < maxThird {
					l = mid
				} else {
					r = mid - 1
				}
			}

			if nums[r] < maxThird {
				count = count + r - j
			}

		}
	}

	return count
}

func sort(arr []int) {
	quicksort(arr, 0, len(arr)-1)
}
func quicksort(arr []int, l, r int) {
	if l >= r {
		return
	}

	p := potition(arr, l, r)
	quicksort(arr, l, p)
	quicksort(arr, p+1, r)
}
func potition(arr []int, l, r int) int {
	target := arr[l]

	j := l

	for i := l + 1; i <= r; i++ {
		if arr[i] < target {
			arr[i], arr[j+1] = arr[j+1], arr[i]
			j++
		}
	}

	arr[j], arr[l] = arr[l], arr[j]
	return j
}

func main() {

	a := []int{16, 24, 29, 6, 48, 24, 44, 27, 7, 6, 17, 51, 37, 19, 23, 0, 20, 42, 49, 41, 46, 35, 24, 52, 47, 32, 1, 14, 30, 51, 30, 38, 2, 30, 14, 27, 4, 23, 10, 11, 50, 52, 4, 41, 13, 54, 32, 6, 44, 48, 16, 26, 28, 51, 34, 24, 19, 31, 45, 40, 44, 0, 25, 11, 20, 33, 37, 15, 43, 41, 20, 38, 54, 35, 43, 53, 13, 20, 6, 20, 0, 34, 54, 54, 20, 20, 32, 3, 9, 9, 1, 54, 11, 24, 51, 42, 29, 41, 3, 48, 5, 11, 33, 40, 52, 25, 4, 12, 18, 0, 2, 1, 23, 38, 3, 44, 34, 11, 29, 51, 40, 53, 5, 13, 51, 24, 4, 0, 13, 9, 2, 52, 50, 43, 1}
	aa := a[:]

	fmt.Println(triangleNumber(aa))
}
