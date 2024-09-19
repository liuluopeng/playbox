package main

import "fmt"

func detectCapitalUse(word string) bool {
	UP := 0

	for _, v := range word {
		if v >= 65 && v <= 90 {
			UP++
		}
	}

	fmt.Println(UP)

	if word[0] >= 65 && word[0] <= 90 && UP == 1 {
		return true
	} else if UP > 1 && UP != len(word) {
		return false
	} else if UP == len(word) {
		return true
	} else if UP == 0 {
		return true
	} else {
		return false
	}

}

func main() {
	fmt.Println(detectCapitalUse("f"))
}
