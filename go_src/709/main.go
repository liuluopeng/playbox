package main

import "fmt"

func toLowerCase(str string) string {

	copyStr := []rune(str)

	for i, ch := range str {
		switch ch {
		case 'A':
			copyStr[i] = 'a'
		case 'B':
			copyStr[i] = 'b'
		case 'C':
			copyStr[i] = 'c'
		case 'D':
			copyStr[i] = 'd'
		case 'E':
			copyStr[i] = 'e'
		case 'F':
			copyStr[i] = 'f'
		case 'G':
			copyStr[i] = 'g'
		case 'H':
			copyStr[i] = 'h'
		case 'I':
			copyStr[i] = 'i'
		case 'J':
			copyStr[i] = 'j'
		case 'K':
			copyStr[i] = 'k'
		case 'L':
			copyStr[i] = 'l'
		case 'M':
			copyStr[i] = 'm'
		case 'N':
			copyStr[i] = 'n'
		case 'O':
			copyStr[i] = 'o'
		case 'P':
			copyStr[i] = 'p'
		case 'Q':
			copyStr[i] = 'q'
		case 'R':
			copyStr[i] = 'r'
		case 'S':
			copyStr[i] = 's'
		case 'V':
			copyStr[i] = 'v'
		case 'T':
			copyStr[i] = 't'
		case 'U':
			copyStr[i] = 'u'
		case 'W':
			copyStr[i] = 'w'
		case 'X':
			copyStr[i] = 'x'
		case 'Y':
			copyStr[i] = 'y'
		case 'Z':
			copyStr[i] = 'z'

		}

	}

	return string(copyStr)
}

func main() {

	fmt.Println(toLowerCase("Hello World"))
}
