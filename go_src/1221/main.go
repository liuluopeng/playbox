package main

func balancedStringSplit(s string) int {
    count := 0
    
    balance := 0
    for _,char := range s{
        if char == 'R'{
            balance += 1
        } else {
            balance -= 1
        }
        if balance == 0{
            count ++
        }
    }
    
    return count
}