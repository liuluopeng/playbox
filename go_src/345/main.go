package main

import "fmt"

func reverseVowels(s string) string {
	sbyte := []byte(s)

	// if isVowel(sbyte[j]) && isVowel(sbyte[i]) {
	// 	fmt.Printf("%d %c  %d %c\n", i, sbyte[i], j, sbyte[j])
	// 	sbyte[i], sbyte[j] = sbyte[j], sbyte[i]

	// }

	vowelIndex := make([]int, 0)

	for i, v := range sbyte {
		if isVowel(v) {
			vowelIndex = append(vowelIndex, i)
		}
	}

	// fmt.Println(vowelIndex)

	for i := 0; i < len(vowelIndex)/2; i++ {
		// fmt.Printf("%c %c\n", sbyte[vowelIndex[i]], sbyte[vowelIndex[len(vowelIndex)-1-i]])
		sbyte[vowelIndex[i]], sbyte[vowelIndex[len(vowelIndex)-1-i]] = sbyte[vowelIndex[len(vowelIndex)-1-i]], sbyte[vowelIndex[i]]

	}

	return string(sbyte)
}

func isVowel(ch byte) bool {
	if ch == 'a' || ch == 'a'-32 || ch == 'e' || ch == 'e'-32 || ch == 'i' || ch == 'i'-32 || ch == 'o' || ch == 'o'-32 || ch == 'u' || ch == 'u'-32 {
		return true
	} else {
		return false
	}
}

func main() {
	fmt.Println(reverseVowels("leetcode"))
}
