package main

import "fmt"

func titleToNumber(s string) int {

	count := 0
	for _, char := range s {
		count *= 26
		count += int(char) - 64
	}

	return count
}

func main() {
	fmt.Println(titleToNumber("ZY"))
}
