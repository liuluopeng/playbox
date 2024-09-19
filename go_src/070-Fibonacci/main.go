package main

import "fmt"

func climbStairs(n int) int {
	// 0, 1, 2, 3, 4
	// 0, 1, 1, 2, 3,
	front, back := 0, 1
	for time := 1; time <= n; time++ {
		front, back = back, front+back
	}

	// 这里 front是fib(n)  back是fib(n+1)
	return back
}

func main() {
	fmt.Println(climbStairs(3))
}
