package main

import (
	"fmt"
	"strings"
)

func wordPattern(pattern string, str string) bool {

	if pattern == str {
		if len(str) == 1 {
			return false
		} else {
			return true
		}
	}

	words := strings.Split(str, " ")
	pat := []byte(pattern)

	patalphas := make([]byte, 0)
	for _, v1 := range pat {
		exits := false
		for _, v2 := range patalphas {
			if v1 == v2 {
				exits = true
				break
			}
		}
		if exits == false {
			patalphas = append(patalphas, v1)
		}
	}

	fmt.Println("XYZ", patalphas)

	isSame := ""
	for _, word := range words {
		isSame += word
	}

	if pattern == isSame {
		return true
	}

	m := make(map[byte][]int)

	alpha := make([]byte, 26)
	for i := 0; i < 26; i++ {
		alpha[i] = byte(i + 65)
	}

	fmt.Println(alpha)

	onewords := make([]string, 0)
	for _, word1 := range words {
		exist := false
		for _, word2 := range onewords {
			if word2 == word1 {
				exist = true
				break
			}
		}
		if exist == false {
			onewords = append(onewords, word1)
		}
	}

	if len(onewords) != len(patalphas) {
		return false
	}

	for i, word := range words {

		pos := 0
		for i, word1 := range onewords {
			if word1 == word {
				pos = i
			}
		}

		fmt.Println(i, word)
		m[patalphas[pos]] = append(m[patalphas[pos]], i)
	}

	fmt.Println(m)

	res := make([]byte, len(words))
	for k := range m {
		arr := m[k]
		for _, v := range arr {
			res[v] = k
		}
	}

	fmt.Println(res)

	fmt.Println()
	fmt.Println(pat)
	fmt.Println(res)
	fmt.Println()

	if len(pat) == len(res) {
		dis := pat[0] - res[0]
		for i := 0; i < len(pat); i++ {
			if pat[i]-res[i] != dis {
				return false
			}

		}
	} else {
		return false
	}

	return true
}

func main() {

	pattern := "abba"
	str := "dog cat cat fish"

	fmt.Println(wordPattern(pattern, str))
}
