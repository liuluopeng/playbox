package main

import "fmt"

func myAtoi2(str string) int {
	res := 0

	// 排除非数字前缀
	for _, v := range str {

		if v != '-' && v != ' ' {
			if v < 47 || v > '9' {
				return 0
			}
		}

	}

	strRes := ""

	for _, v := range str {
		if v <= '0' || v >= '9' {
			break
		}
		strRes += string(v)
	}

	fmt.Println("strRes", strRes)

	return res
}
