package main

import "fmt"

func main() {

	score := [][]int{
		{10, 6, 9, 1}, {7, 5, 11, 2}, {4, 8, 3, 15},
	}
	k := 2;
	fmt.Println(sortTheStudents(score, k))
}
func sortTheStudents(score [][]int, k int) [][]int {
	quickSortBy1Subj(score, 0, len(score)-1, k)	
	return score 
}
func quickSortBy1Subj(score [][]int, l,r int, k int) {
	if l>r {
		return 
	}
	pivot := score[r][k]
	waitForBigger := l 	
	for i:=l;i<r;i++ {
		if score[i][k] > pivot {
			score[i], score[waitForBigger] = score[waitForBigger], score[i]
			waitForBigger ++ 
		}
	}

	score[waitForBigger], score[r] = score[r], score[waitForBigger]

	quickSortBy1Subj(score, l, waitForBigger - 1, k)
	quickSortBy1Subj(score , waitForBigger + 1, r, k)
}