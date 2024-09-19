package main

import "fmt"

func hasAlternatingBits(n int) bool {

	if n == 3 {
		return false
	}

	// odd: 1010101
	if n%2 == 1 {
		n /= 2
		for n > 2 {
			if n%4 == 2 {
				n /= 4
			} else {
				return false
			}
		}
		if n == 1 {
			return false
		}
	}

	// nod odd: 10101010
	if n%2 == 0 {
		for n > 2 {
			if n%4 == 2 {
				n /= 4
			} else {
				return false
			}
		}

		if n == 1 {
			return false
		}
	}

	return true
}

func main() {
	fmt.Println(hasAlternatingBits(53))
}
