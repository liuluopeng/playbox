package main

import "fmt"

func judgeCircle(moves string) bool {
	x := 0
	y := 0
	for _, char := range moves {
		if char == 'R' {
			x++
		} else if char == 'L' {
			x--
		} else if char == 'U' {
			y++
		} else {
			y--
		}
	}

	return x == 0 && y == 0
}

func main() {
	fmt.Println(judgeCircle("RL"))
}
