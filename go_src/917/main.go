package main

import (
	"fmt"
)

func reverseOnlyLetters(S string) string {
	st := []byte(S)

	indexes := make([]int, 0)

	for i, ch := range st {
		if (ch >= 65 && ch <= 65+25) || (ch <= 122 && ch >= 122-25) {

			indexes = append(indexes, i)
		}
	}

	fmt.Println(indexes)

	for i := 0; i < len(indexes)/2; i++ {
		fmt.Printf("%c %c\n", st[indexes[i]], st[indexes[len(indexes)-1-i]])
		fmt.Println(string(st))
		st[indexes[i]], st[indexes[len(indexes)-1-i]] = st[indexes[len(indexes)-1-i]], st[indexes[i]]

		fmt.Println()
	}

	return string(st)
}

func main() {
	S := "aa-bC_dEf-ghIj"
	fmt.Println(reverseOnlyLetters(S))
}
