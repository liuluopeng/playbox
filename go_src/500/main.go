// bype uint8
// rune int32


package main

import "fmt"

func findWords(words []string) []string {

	line := make(map[rune]int)

	res := []string{}

	for _, ch := range "qwertyuiop" {
		line[ch] = 0
		line[rune(ch-32)] = 0
	}
	for _, ch := range "asdfghjkl" {
		line[ch] = 1
		line[rune(ch-32)] = 1
	}
	for _, ch := range "zxcvbnm" {
		line[ch] = 2
		line[rune(ch-32)] = 2
	}

	for _, word := range words {

		copyword := []rune(word)

		SAMELINE := false
		for i := 0; i < len(word)-1; i++ {
			if line[copyword[i]] != line[copyword[i+1]] {
				SAMELINE = true
			}

		}

		if SAMELINE != true {
			res = append(res, word)
		}
	}

	return res
}

func main() {

	words := []string{"Hello", "Alaska", "Dad", "Peace"}

	fmt.Println(findWords(words))


	a := "HELLO WORLD"
	b := []byte(a)
	fmt.Println(b)
}
