package main

import "fmt"

func strStr(haystack string, needle string) int {

	if len(needle) == 0 {
		return 0
	}

	sh := []byte(haystack)
	sn := []byte(needle)

	for i := len(needle) - 1; i < len(haystack); i++ {
		var same bool
		count := 0

		for j := i - len(needle) + 1; j <= i; j++ {

			fmt.Println("hay", sh[j], "needle", sn[count])

			if sh[j] != sn[count] {
				same = false
				break
			} else {
				count++
			}
		}

		if count == len(needle) {
			same = true
		}

		if same == true {
			return i - len(needle) + 1
		}
	}

	return -1

}

func main() {
	fmt.Println(strStr("aaaaa", "bba"))
}
