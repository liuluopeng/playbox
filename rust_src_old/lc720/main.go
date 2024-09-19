package main

import "fmt"

func main() {
	st := []string{"a", "banana", "app", "appl", "ap", "apply", "apple"}
	st = []string{"m", "mo", "moc", "moch", "mocha", "l", "la", "lat", "latt", "latte", "c", "ca", "cat"}

	st = []string{"ogz", "eyj", "e", "ey", "hmn", "v", "hm", "ogznkb", "ogzn", "hmnm", "eyjuo", "vuq", "ogznk", "og", "eyjuoi", "d"}

	st = []string{"wo", "wor", "worl", "world"}

	fmt.Println(longestWord(st))
}

// 有无  逐步添加 一个字母 组成的单词?
func longestWord(words []string) string {

	tire := &TireNode{}

	for _, word := range words {
		tire.Insert(word)
	}

	res := tire.FindLong()

	fmt.Println(res)

	maxL := -1
	for _, r := range res {
		if len(r) > maxL {
			maxL = len(r)
		}
	}


	for _, r := range res {
		if len(r) == maxL {
			return r
		}
	}

	return ""
}

type TireNode struct {
	Next   [26]*TireNode
	IsWord bool
}

func (this *TireNode) Insert(word string) {
	currentNode := this

	for _, char := range word {

		index := char - 'a'
		// fmt.Println(index)
		if currentNode.Next[index] == nil {
			currentNode.Next[index] = &TireNode{}
		}
		currentNode = currentNode.Next[index]
	}

	currentNode.IsWord = true
}

func (this *TireNode) FindWord(word string) bool {
	currentNode := this

	for _, char := range word {
		index := char - 'a'
		if currentNode.Next[index] == nil {
			return false
		}
		currentNode = currentNode.Next[index]
	}

	return currentNode.IsWord
}

func (this *TireNode) FindLong() []string {
	currentNode := this
	res := []string{}
	var dfs func(currentNode *TireNode, word string)
	dfs = func(currentNode *TireNode, word string) {
		for i, node := range currentNode.Next {
			if node != nil {

				if node.IsWord {
					ch := byte(i + 97)
					// fmt.Println(ch)
					newWord := word + string(ch)

					res = append(res, newWord)

					dfs(node, newWord)
				} else {
					// ch := byte(i + 97)
					// newWord := word + string(ch)
					// dfs(node, newWord)
				}

			}
		}

	}

	dfs(currentNode, "")
	return res
}
