package main

import "fmt"

func getLen(n int) int {
	copy := n
	count := 0
	for copy > 0 {
		copy /= 10
		count++
	}
	return count
}

func has1(n int) int {
	copy := n
	count := 0
	t := 0
	for copy > 0 {

		t = copy % 10
		if t == 1 {
			count++
		}
		copy /= 10
	}

	return count
}

func countDigitOne2(n int) int {

	count := 0

	for i := 1; i <= n; i++ {
		count += has1(i)
	}

	return count
}

func countDigitOne(n int) int {

	if n < 1 {
		return 0
	}
	count := 0
	for i := 1; i <= n; i++ {
		num := i
		for num != 0 {
			if num%10 == 1 {
				count++
			}
			num = num / 10
		}
	}
	return count
}

func main() {
	fmt.Println(countDigitOne(824883294))
}
