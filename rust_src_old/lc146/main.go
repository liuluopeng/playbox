package main

import "container/list"

type LRUCache struct {
	Capacity int
	list     *list.List
}

func Constructor(capacity int) LRUCache {

	return LRUCache{
		Capacity: capacity,
	}

}

func (this *LRUCache) Get(key int) int {

	// 放到第一个

	return -1
}

func (this *LRUCache) Put(key int, value int) {
	// 看看是否存在

}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */

func main() {
	obj := Constructor(capacity)
	param_1 := obj.Get(key)
	obj.Put(key, value)
}
