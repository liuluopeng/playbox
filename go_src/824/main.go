package main

import (
	"fmt"
	"strings"
)

func toGoatLatin(S string) string {
	res := ""

	sslice := strings.Split(S, " ")
	fmt.Println(sslice, len(sslice))


	goat := make([]string, len(sslice))


	for i, word := range sslice {
		if isVowel(word[0]) == true {
			goat[i] = word + "ma"

			for numsa := i+1;numsa>0;numsa--{
				goat[i] = goat[i] + "a"
			}
		} else {
			goat[i] = word[1:]
			goat[i] += string(word[0]) + "ma"
			for numsa := i+1;numsa>0;numsa--{
				goat[i] = goat[i] + "a"
			}

		}
	}

	fmt.Println(goat, len(goat))

	for i, word := range goat {
		if i != len(goat)-1 {
			res += word + " "
		} else {
			res += word
		}
	}

	return res
}

func isVowel(ch byte) bool {
	if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' || ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U' {
		return true
	}

	return false
}

func main() {
	S := "I speak Goat Latin"
	fmt.Println(toGoatLatin(S))
}
