package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	contents, _ := ioutil.ReadFile("input.txt")
	sContents := string(contents)

	sDepths := strings.Split(sContents, "\n")

	depths := make([]int, 0)

	for _, sDepth := range sDepths {
		if sDepth != "" {
			num, _ := strconv.Atoi(sDepth)
			depths = append(depths, num)
		}
	}

	// start bigger than the input start so this properly tracks
	lastNum := 10000000
	increased := 0

	for _, d := range depths {
		if d > lastNum {
			increased++
		}

		lastNum = d
	}

	fmt.Println("part one solution", increased)

	// reset increased
	increased = 0
	max := len(depths) - 3

	for i := 0; i < max; i++ {
		if depths[i+3] > depths[i] {
			increased++
		}
	}

	fmt.Println("part two solution", increased)
}
