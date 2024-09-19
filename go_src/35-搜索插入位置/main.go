func searchInsert(nums []int, target int) int {
    for i,v := range nums{
        if v>=target{
            return i
        }
    }

    // targat 比数组最大的还要大
    return len(nums)
}