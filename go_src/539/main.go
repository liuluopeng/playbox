package main

import (
	"fmt"
	"sort"
	"strconv"
)

func abs(x, y int) int {
	if x-y < 0 {
		return -(x - y)
	}
	return x - y
}

func findMinDifference(timePoints []string) int {
	times := make([]int, len(timePoints))

	for i, time := range timePoints {

		hour, _ := strconv.Atoi(time[:2])

		mini, _ := strconv.Atoi(time[3:])

		times[i] = hour*60 + mini
	}



	sort.Ints(times)

	fmt.Println(times)
	minTime := abs(times[0], times[1])
	fmt.Println(minTime)


	for i := 1; i < len(times)-1; i++ {
		if abs(times[i], times[i+1]) < minTime {
			minTime = abs(times[i], times[i+1])
		}
	}

	if times[0]+1440-times[len(times)-1] < minTime {
		fmt.Println(minTime)
		minTime = times[0] + 1440 - times[len(times)-1]
		fmt.Println(minTime)
	}

	return minTime
}

func main() {

	times := []string{"23:59", "00:00", "00:00"}
	timess := times[:]
	fmt.Println(findMinDifference(timess))
}
