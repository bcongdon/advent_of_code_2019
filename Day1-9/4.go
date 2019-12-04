package main

import (
	"fmt"
	"strconv"
)

func isSorted(s string) bool {
	for i := 1; i < len(s); i++ {
		if s[i] < s[i-1] {
			return false
		}
	}
	return true
}

func passwordGuess(a, b int) (int, int) {
	p1, p2 := 0, 0
	for x := a; x <= b; x++ {
		xStr := strconv.Itoa(x)
		if !isSorted(xStr) {
			continue
		}
		digits := make(map[byte]int)
		for i := 0; i < len(xStr); i++ {
			digits[xStr[i]]++
		}
		for _, v := range digits {
			if v >= 2 {
				p1++
				break
			}
		}
		for _, v := range digits {
			if v == 2 {
				p2++
				break
			}
		}
	}
	return p1, p2
}

func main() {
	p1, p2 := passwordGuess(236491, 713787)

	fmt.Printf("Part 1: %d\n", p1)
	fmt.Printf("Part 2: %d\n", p2)
}
