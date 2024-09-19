package main

import (
	"fmt"
)

func largestTimeFromDigits(A []int) string {

	// 无效次数
	count := 0

	Maxtime := 0
	MaxA := [4]int{}

	// 0 1 2 3 的全排列
	permutations := [24][4]int{
		{0, 1, 2, 3},
		{0, 1, 3, 2},
		{0, 2, 1, 3},
		{0, 2, 3, 1},
		{0, 3, 2, 1},
		{0, 3, 1, 2},
		{1, 0, 2, 3},
		{1, 0, 3, 2},
		{1, 3, 2, 0},
		{1, 3, 0, 2},
		{1, 2, 3, 0},
		{1, 2, 0, 3},
		{2, 0, 1, 3},
		{2, 0, 3, 1},
		{2, 1, 3, 0},
		{2, 1, 0, 3},
		{2, 3, 0, 1},
		{2, 3, 1, 0},
		{3, 1, 0, 2},
		{3, 1, 2, 0},
		{3, 2, 0, 1},
		{3, 2, 1, 0},
		{3, 0, 1, 2},
		{3, 0, 2, 1},
	}

	for _, v := range permutations {
		// 小时是否大于24
		if A[v[0]]*10+A[v[1]] > 23 || A[v[2]]*10+A[v[3]] > 59 {
			count++
			continue
		} else {
			if (A[v[0]]*10+A[v[1]])*60+A[v[2]]*10+A[v[3]] > Maxtime {
				Maxtime = (A[v[0]]*10+A[v[1]])*60 + A[v[2]]*10 + A[v[3]]
				MaxA = [4]int{A[v[0]], A[v[1]], A[v[2]], A[v[3]]}
			}
		}
	}

	if count == 24 {
		return ""
	}

	// return strconv.Itoa(Maxtime/60) + ":" + strconv.Itoa(Maxtime%60)

	return fmt.Sprintf("%d%d:%d%d", MaxA[0], MaxA[1], MaxA[2], MaxA[3])
}

func main() {
	A := []int{1, 9, 2, 3}
	fmt.Println(largestTimeFromDigits(A))
}
