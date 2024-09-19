package main

import "fmt"

func main() {
	/**
	 * Your MagicDictionary object will be instantiated and called as such:
	 * obj := Constructor();
	 * obj.BuildDict(dictionary);
	 * param_2 := obj.Search(searchWord);
	 */

	obj := Constructor()
	dic := []string{"hello", "world"}
	obj.BuildDict(dic)
	res := obj.Search("hello")
	fmt.Println(res)
}

type MagicDictionary struct {
	Root *TireNode
}

type TireNode struct {
	Next   [26]*TireNode
	IsWord bool
}

func Constructor() MagicDictionary {
	return MagicDictionary{
		Root: &TireNode{},
	}
}

func (this *MagicDictionary) Insert(word string) {
	currentNode := this.Root

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

func (this *MagicDictionary) FindWord(word string) bool {
	currentNode := this.Root

	for _, char := range word {
		index := char - 'a'
		if currentNode.Next[index] == nil {
			return false
		}
		currentNode = currentNode.Next[index]
	}

	return currentNode.IsWord
}

func (this *MagicDictionary) BuildDict(dictionary []string) {
	// currentNode := this.Root

	for _, word := range dictionary {
		this.Insert(word)
	}
}

func (this *MagicDictionary) Search(searchWord string) bool {
	// //  给一次不一样的机会.   不能两次不一样  这一次机会必须使用掉

	// se := []byte(searchWord)

	// var dfs func()
	// dfs = func() {

	// }

	for i := 0; i < len(searchWord); i++ {
		but := searchWord[i] - 'a'

		for j := 0; j < 26; j++ {
			if j != int(but) {
				new_word := []byte(searchWord)
				new_word[i] = byte(j) + 'a'
				// fmt.Println(string(new_word))

				if this.FindWord(string(new_word)) {
					return true
				}
			}

		}

		// fmt.Println()

	}

	return false
}
