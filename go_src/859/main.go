package main

import "fmt"

func buddyStrings(A string, B string) bool {

	sA := []byte(A)
	sB := []byte(B)

	diffA := make([]int, 0)

	if A == "" {
		return false
	}

	if len(sA) != len(sB) {
		return false
	}

	for i := 0; i < len(sA); i++ {
		if sA[i] != sB[i] {
			diffA = append(diffA, i)
		}
	}

	if len(diffA) != 2 {
		if len(diffA) == 0 {
			// 每个都不一样
			m := make(map[byte]int)
			for _, ch := range sA {
				m[ch]++
			}

			for k := range m {
				if m[k] != 1 {
					return true
				}
			}
			return false
		} else if len(diffA) > 2 || len(diffA) == 1 {
			return false
		}
	}

	if sA[diffA[0]] == sB[diffA[1]] && sA[diffA[1]] == sB[diffA[0]] {
		return true
	}

	return false
}

func main() {
	fmt.Println(buddyStrings("abcc", "abcc"))
}
