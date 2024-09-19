func average(salary []int) float64 {

    n := len(salary)
    if n > 2{
        quickSort(salary,0,n-1)
        salary = salary[1:n-1]
    }
    var sum float64 
    sum = 0.0
    for _,moeny := range salary{
        sum += float64(moeny)
    }
    return sum/float64(len(salary))
}

func quickSort(salary []int,start int,end int){

    if start > end{
        return
    }
    counter,pivot := start,end
    for i := start;i<end;i++{
        if salary[i] < salary[pivot]{
            salary[i],salary[counter] = salary[counter],salary[i]
            counter++
        }
    }
    salary[counter],salary[pivot] = salary[pivot],salary[counter]
    quickSort(salary,start,counter-1)
    quickSort(salary,counter+1,end)
}