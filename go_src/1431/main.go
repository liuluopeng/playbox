func kidsWithCandies(candies []int, extraCandies int) []bool {
    theMax := 0
    for _,e := range candies{
        if theMax < e {
            theMax = e
        }
    }
    
    res := make([]bool, len(candies))
    
    for i,e := range candies{
        if e + extraCandies >= theMax{
            res[i] = true   
        }
    }
    
    return res
}