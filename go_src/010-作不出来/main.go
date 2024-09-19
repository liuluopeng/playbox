package main

import "fmt"

func isMatch(s string, p string) bool {

	length := len(s)



	return length == 0
}

func main() {
	s := "aa"
	p := "a"
	fmt.Println(isMatch(s, p))
}
