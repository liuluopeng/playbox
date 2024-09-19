package main

import "fmt"

func reverseWords(s string) string {

	S := []rune(s)

	words := make([][]rune, 0)
	word := []rune{}

	
	for _, v := range S {
		if v != ' ' {
			word = append(word, v)
		} else {
			words = append(words, word)
			fmt.Println(word)
			word = word[0:0]
		}
	}

	res := []rune{}
	for _, word := range words {
		for i := len(word) - 1; i >= 0; i-- {
			res = append(res, word[i])
		}
		res = append(res, ' ')

	}

	fmt.Printf("%v\n", res)
	return string(res)
}

func main() {
	s := "Let's take LeetCode contest"

	fmt.Println(reverseWords(s))
}
