package main

import (
	"fmt"
	"strings"
)

func myAtoi(str string) int {

	if len(str) == 0 {
		return 0
	}

	res := 0

	slice := strings.Split(str, " ")

	numStr := ""
	for _, word := range slice {
		if word != "" {
			numStr = word
			break
		}
	}

	fmt.Println("numStr", len(numStr), numStr)

	if len(numStr) == 0 {
		return 0
	}

	intslice := cutNum(numStr)

	fmt.Println("istslice", intslice)

	if len(intslice) == 0 {
		return 0
	} else {
		if numStr[0] == '+' {
			res = atoi('+', numStr[1:])
			fmt.Println("++", res)
		} else if numStr[0] == '-' {
			res = atoi('-', numStr[1:])
			fmt.Println("--", res)
			fmt.Println(numStr[1:])
		} else {
			res = atoi(' ', numStr)
			fmt.Println("**", res)
		}
	}

	return res
}

func cutNum(s string) string {
	// first char
	firstchar := s[0]
	if firstchar != '+' && firstchar != '-' && firstchar != '1' && firstchar != '2' && firstchar != '3' && firstchar != '4' && firstchar != '5' && firstchar != '6' && firstchar != '7' && firstchar != '8' && firstchar != '9' && firstchar != '0' {
		return ""
	}

	res := ""

	if firstchar == '+' || firstchar == '-' {
		for i := 1; i < len(s); i++ {
			if s[i] != '1' && s[i] != '2' && s[i] != '3' && s[i] != '4' && s[i] != '5' && s[i] != '6' && s[i] != '7' && s[i] != '8' && s[i] != '9' && s[i] != '0' && s[i] != '.' {
				return res
			} else {
				res += string(s[i])
			}
		}
	} else {
		for i := 0; i < len(s); i++ {
			if s[i] != '1' && s[i] != '2' && s[i] != '3' && s[i] != '4' && s[i] != '5' && s[i] != '6' && s[i] != '7' && s[i] != '8' && s[i] != '9' && s[i] != '0' && s[i] != '.' {
				return res
			} else {
				res += string(s[i])
			}
		}
	}

	return res
}

func atoi(postive byte, s string) int {
	res := 0
	for i := 0; i < len(s); i++ {

		intch := 0
		if s[i] == '0' {
			intch = 0
		} else if s[i] == '1' {
			intch = 1
		} else if s[i] == '2' {
			intch = 2
		} else if s[i] == '3' {
			intch = 3
		} else if s[i] == '4' {
			intch = 4
		} else if s[i] == '5' {
			intch = 5
		} else if s[i] == '6' {
			intch = 6
		} else if s[i] == '7' {
			intch = 7
		} else if s[i] == '8' {
			intch = 8
		} else if s[i] == '9' {
			intch = 9
		} else {
			return res
		}

		// -234
		if postive == '-' {
			res = res*(10) - intch
			if res < -2147483648 {
				fmt.Println("too smail", res)
				res = -2147483648
				return res
			}
		} else if postive == '+' || postive == ' ' {
			res = res*10 + intch
			fmt.Println("every time", res)
			if res > 2147483647 {
				res = 2147483647
				return res
			}
		}
	}

	return res
}

func main() {

	str := "   -42"
	str = "4193 with words"
	str = "words and 987"
	str = "-91283472332"
	str = "3.14159"
	str = " "
	str = ""
	str = "-0012a42"
	str = "1"
	str = "9223372036854775808"
	str = "-91283472332"

	fmt.Println("Final ", myAtoi(str))
}
