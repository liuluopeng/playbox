

package main


func singleNumber(nums []int) int {
    res := 0
    for _,v := range nums {
        res ^= v
    }
    
    return res
}

func singleNumber2(nums []int) int {
    m := make(map[int]int)
    for _, v := range nums {
        m[v] ++
    }
    
    for k := range m {
        if m[k] == 1{
            return k
        }
    }
    
    return -1
}

func main(){
	
}