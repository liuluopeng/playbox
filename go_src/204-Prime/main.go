package main

import "fmt"

func isPrime(n int) bool {
	for i := 2; i < n/2; i++ {
		if n%i == 0 {
			return false
		}
	}

	return true
}

func countPrimes(n int) int {
	if n == 0 || n == 1 || n == 2 {
		return 0
	}
	if n == 3 {
		return 1
	}

	count := 1

	for i := 3; i < n; i += 2 {
		if isPrime(i) {
			fmt.Println(i)
			count++
		}
	}

	return count
}

func main() {
	fmt.Println(isPrime(9))
	fmt.Println(countPrimes(10))
	// fmt.Println(countPrimes(499979))
}
