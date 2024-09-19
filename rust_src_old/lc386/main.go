package main

import (
	"fmt"
	"strconv"
)

type Tire struct {
	Node  [10]*Tire // [0,1,2,3,4,5,6,7,8,9]
	IsEnd bool
}

func reverseNumberWithZero(num int) string {
	strNum := strconv.Itoa(num)
	reversed := ""
	for i := len(strNum) - 1; i >= 0; i-- {
		reversed += string(strNum[i])
	}
	return reversed
}

func (t *Tire) Insert(num int) {

	// if t.Have(num) {
	// 	return
	// }

	current := t
	num_st := strconv.Itoa(num)

	for _, char := range num_st {
		dig, _ := strconv.Atoi(string(char))
		// fmt.Println(dig)
		if current.Node[dig] == nil {
			current.Node[dig] = &Tire{}
		}

		current = current.Node[dig]
	}

	current.IsEnd = true
}

func (t Tire) Have(num int) bool {
	current := t
	num_st := strconv.Itoa(num)

	for _, char := range num_st {
		dig, _ := strconv.Atoi(string(char))
		if current.Node[dig] == nil {
			return false
		}
		current = *current.Node[dig]
	}

	if current.IsEnd == true {
		return true
	}
	return false
}

func (t Tire) Save2Arr() []int {
	res := []int{}

	current := t
	var dfs func(current *Tire, sum int)
	dfs = func(current *Tire, sum int) {
		for i, v := range current.Node {
			if v != nil {

				newSum := sum*10 + i

				if v.IsEnd == true {
					res = append(res, newSum)
				}
				dfs(current.Node[i], newSum)

			}
		}
	}

	dfs(&current, 0)

	return res
}

func lexicalOrder(n int) []int {
	tire := &Tire{}
	for i := 1; i <= n; i++ {
		tire.Insert(i)
	}

	return tire.Save2Arr()
}

func main() {

	res := lexicalOrder(100)
	fmt.Println(res)
}
