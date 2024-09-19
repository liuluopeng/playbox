package main

import "fmt"

type TireNode struct {
	Level [26]*TireNode
	IsEnd bool
}

type WordDictionary struct {
	Root *TireNode
}

func Constructor() WordDictionary {
	return WordDictionary{
		Root: &TireNode{},
	}
}

func (this *WordDictionary) AddWord(word string) {
	currentNode := this.Root

	for _, char := range word {
		index := char - 'a'
		if currentNode.Level[index] == nil {
			currentNode.Level[index] = &TireNode{}
		}
		currentNode = currentNode.Level[index]
	}

	currentNode.IsEnd = true
}

// ["bad"],[".ad"],["b.."]
func (this *WordDictionary) Search(word string) bool {
	word_byte := []byte(word)

	currentNode := this.Root
	res := []string{}
	var dfs func(node *TireNode, level int, wordNow string)
	dfs = func(node *TireNode, level int, wordNow string) {
		if level == len(word) {
			return
		}

		for i, n := range node.Level {
			if n != nil {
				char := byte(i + 97)

				if word_byte[level] == 46 { // 这里是一个点

					newWord := wordNow + string(char)
					if level == len(word)-1 && n.IsEnd {
						res = append(res, newWord)
					}

					dfs(n, level+1, newWord)

				} else { // 这里要求必须与字母一致

					if char == word_byte[level] {

						newWord := wordNow + string(char)
						if level == len(word)-1 && n.IsEnd {
							res = append(res, newWord)
						}

						dfs(n, level+1, newWord)

					}

				}

			}
		}
	}

	dfs(currentNode, 0, "")

	fmt.Println(res)

	return len(res) >= 1
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AddWord(word);
 * param_2 := obj.Search(word);
 */

func main() {

	obj := Constructor()
	obj.AddWord("bad")
	obj.AddWord("dad")

	res := obj.Search("...")
	fmt.Println(res)
}
