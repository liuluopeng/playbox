package main

import "fmt"

// 只包含小写字母的前缀树

type Letter struct {
	NextCadiRef [26]*Letter
	IsEnd       bool // 是某一个单词的结尾.
}

type Trie struct {
	Root *Letter
}

func Constructor() Trie {
	return Trie{
		Root: &Letter{},
	}
}

func (this *Trie) Insert(word string) {
	if this.Search(word) == true {
		return
	}

	currentLetter := this.Root
	for _, char := range word {

		index := char - 97

		if currentLetter.NextCadiRef[index] == nil { //
			currentLetter.NextCadiRef[index] = &Letter{}
		}

		currentLetter = currentLetter.NextCadiRef[index]
	}

	currentLetter.IsEnd = true
}

func (this *Trie) Search(word string) bool {
	currentLetter := this.Root

	for _, char := range word {
		index := char - 97

		if currentLetter.NextCadiRef[index] == nil {
			return false
		} else {

			currentLetter = currentLetter.NextCadiRef[index]
		}
	}

	if currentLetter.IsEnd == false {
		return false 
	} else {
		return true 
	}


}

func (this *Trie) StartsWith(prefix string) bool {
	// startwith 比 search容易实现

	currentLetter := this.Root
	for _, char := range prefix {
		index := char - 97
		if currentLetter.NextCadiRef[index] == nil {
			return false
		} else {
			currentLetter = currentLetter.NextCadiRef[index]
		}
	}
	return true
}

func (this Trie) Trav() { // DFS遍历出所有的单词.
	currentLetter := this.Root

	fmt.Println(currentLetter.NextCadiRef)

	words := []string{}
	var dfs func(letter *Letter, word string)
	dfs = func(letter *Letter, word string) {

		for i, l := range letter.NextCadiRef {
			char := string(rune(i + 97))
			if l != nil {
				// fmt.Println(string(rune(i + 97)))

				newword := word + char


				dfs(l, newword)

				if l.IsEnd {
					words = append(words, newword)
				}

			} else {

			}

		}

	}

	dfs(currentLetter, "")
	fmt.Println(words)
}

/**
 * Your Trie object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Insert(word);
 * param_2 := obj.Search(word);
 * param_3 := obj.StartsWith(prefix);
 */

func main() {
	obj := Constructor()

	obj.Insert("dog")
	obj.Insert("apple")
	obj.Insert("dog")
	obj.Insert("door")




	obj.Trav()


	app := obj.Search("dog")
	fmt.Println(app)


}
