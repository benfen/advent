package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	input := ""
	r1, r2 := 0, 0
	scores := []int{}
	for scanner.Scan() {
		input = scanner.Text()

		// 1,2,3 0,3,6
		// round 2 lose draw win
		switch input {
		case "A X":
			scores = []int{4, 3}
		case "A Y":
			scores = []int{8, 4}
		case "A Z":
			scores = []int{3, 8}
		case "B X":
			scores = []int{1, 1}
		case "B Y":
			scores = []int{5, 5}
		case "B Z":
			scores = []int{9, 9}
		case "C X":
			scores = []int{7, 2}
		case "C Y":
			scores = []int{2, 6}
		case "C Z":
			scores = []int{6, 7}
		}

		r1 += scores[0]
		r2 += scores[1]
	}

	fmt.Println("Part 1: ", r1)
	fmt.Println("Part 2: ", r2)
}
