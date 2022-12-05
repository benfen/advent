package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	stacks1 := [9][]rune{
		{'R', 'G', 'J', 'B', 'T', 'V', 'Z'},
		{'J', 'R', 'V', 'L'},
		{'S', 'Q', 'F'},
		{'Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'},
		{'R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'},
		{'S', 'W', 'T', 'C', 'H', 'F'},
		{'D', 'Z', 'C', 'V', 'F', 'N', 'J'},
		{'L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'},
		{'J', 'B', 'W', 'V', 'P'},
	}

	stacks2 := [9][]rune{
		{'R', 'G', 'J', 'B', 'T', 'V', 'Z'},
		{'J', 'R', 'V', 'L'},
		{'S', 'Q', 'F'},
		{'Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'},
		{'R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'},
		{'S', 'W', 'T', 'C', 'H', 'F'},
		{'D', 'Z', 'C', 'V', 'F', 'N', 'J'},
		{'L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'},
		{'J', 'B', 'W', 'V', 'P'},
	}

	scanner := bufio.NewScanner(f)

	re := regexp.MustCompile("[0-9]+")

	line := ""
	for scanner.Scan() {
		line = scanner.Text()

		// hack because I didn't want to parse the initial puzzle
		if !strings.Contains(line, "move") {
			continue
		}

		// [amountToMove, fromSlice, toSlice]
		numStrings := re.FindAllString(line, -1)
		nums := []int{}
		for _, s := range numStrings {
			num, _ := strconv.Atoi(s)
			nums = append(nums, num)
		}

		// steal out the runes needed, and reslice the array to remove them
		toMove1 := []rune{}
		toMove2 := []rune{}
		for i := 0; i < nums[0]; i++ {
			toMove1 = append(toMove1, stacks1[nums[1]-1][len(stacks1[nums[1]-1])-(i+1)])
			toMove2 = append(toMove2, stacks2[nums[1]-1][len(stacks2[nums[1]-1])-(i+1)])
		}
		stacks1[nums[1]-1] = stacks1[nums[1]-1][:len(stacks1[nums[1]-1])-nums[0]]
		stacks2[nums[1]-1] = stacks2[nums[1]-1][:len(stacks2[nums[1]-1])-nums[0]]

		// add to new chunk
		stacks1[nums[2]-1] = append(stacks1[nums[2]-1], toMove1...)
		for i := len(toMove2) - 1; i >= 0; i-- {
			stacks2[nums[2]-1] = append(stacks2[nums[2]-1], toMove2[i])
		}
	}

	rval := ""
	rval2 := ""
	for i := 0; i < len(stacks1); i++ {
		rval += string(stacks1[i][len(stacks1[i])-1])
		rval2 += string(stacks2[i][len(stacks2[i])-1])
	}

	fmt.Println("Part 1: ", rval)
	fmt.Println("Part 2: ", rval2)
}
