package main

func main() {
	a := 333
	b := 3838
}

func getGreatestCommonDivisor(a int, b int) int {
	if b == 0 {
		return a
	}

	return getGreatestCommonDivisor(b, a % b)
}
