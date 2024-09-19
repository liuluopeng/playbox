package main

func flipAndInvertImage(A [][]int) [][]int {
	for i := range A {
		for _, vj := range A[i] {
			vj = ^vj
		}
	}

	return A
}

func main() {

}
