package main

import "fmt"

func isAnagram2(s string, t string) bool {
	maps := make(map[rune]int)
	mapt := make(map[rune]int)

	for _, v := range s {
		maps[v]++
	}
	for _, v := range t {
		mapt[v]++
	}

	if len(maps) == len(mapt) {
		for k := range maps {
			vt, ok := mapt[k]
			if ok {
				if vt == maps[k] {
					continue
				} else {
					return false
				}
			} else {
				return false
			}
		}
	} else {
		return false
	}

	return true
}

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	dict := make(map[rune]int)
	for _, v := range s {
		dict[v]++
	}
	for _, v := range t {
		dict[v]--
	}
	for k, _ := range dict {
		if dict[k] != 0 {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(isAnagram("a", "bac"))
}
