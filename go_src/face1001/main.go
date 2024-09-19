package main

import "fmt"

func merge(A []int, m int, B []int, n int) {
	aux := make([]int, m+n)

	for i := 0; i < m; i++ {
		aux[i] = A[i]
	}
	for i := 0; i < n; i++ {
		aux[m+i] = B[i]
	}


	fmt.Println(aux)


	if m == 0{
		A = aux
	}

	i := 0
	j := n

	for k := 0; i < m+n; k++ {
		if i > n-1 && j < len(aux) {
			A[k] = aux[j]
			j++
			fmt.Println(1, j, A)
		} else if j == m+n-1 {
			A[k] = aux[i]
			i++
			fmt.Println(2, i, A)
		} else if i < n && j < m+n && aux[i] <= aux[j] {
			A[k] = aux[i]
			i++
			fmt.Println(3, i, A)
		} else if i < n && j < m+n && aux[i] > aux[j] {
			A[k] = aux[j]
			j++
			fmt.Println(4, j, A)
		} else {
			break
		}
	}

	fmt.Println("Final", A)
}

func main() {
	a := []int{0}
	b := []int{1}

	A := a[:]
	B := b[:]

	merge(A, 0, B, 1)
}
