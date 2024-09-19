func numIdenticalPairs(nums []int) int {
    count := 0
    for i,e1 := range nums{
        for j,e2 := range nums{
            if i==j{
                continue
            }else if e1==e2 && i<j{
                count ++
            }
        }
    }
    return count
}