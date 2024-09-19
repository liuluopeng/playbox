func missingNumber(nums []int) int {
    sum := (len(nums)+1) * len(nums) / 2
    for _,v := range nums{
        sum -= v
    }
    return sum
}