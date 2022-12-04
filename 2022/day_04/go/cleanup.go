package main

import (
	"bufio"
	"fmt"
	"os"
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

	scanner := bufio.NewScanner(f)

	containingPairs := 0
	overlapping := 0

	for scanner.Scan() {
		ranges := strings.Split(scanner.Text(), ",")

		elf1 := ElfFromRange(ranges[0])
		elf2 := ElfFromRange(ranges[1])

		if elf1.Contains(elf2) || elf2.Contains(elf1) {
			containingPairs++
		}

		if elf1.Overlaps(elf2) || elf2.Overlaps(elf1) {
			overlapping++
		}
	}

	fmt.Println("Part 1: ", containingPairs)
	fmt.Println("Part 2: ", overlapping)
}

type Elf struct {
	Low  int
	High int
}

func ElfFromRange(s string) Elf {
	brokenDownRange := strings.Split(s, "-")
	low, _ := strconv.Atoi(brokenDownRange[0])
	high, _ := strconv.Atoi(brokenDownRange[1])

	return Elf{Low: low, High: high}
}

func (e Elf) Contains(e2 Elf) bool {
	return e2.Low >= e.Low && e2.High <= e.High
}

func (e Elf) Overlaps(e2 Elf) bool {
	return (e2.Low >= e.Low && e2.Low <= e.High) || (e2.High >= e.Low && e2.High <= e.High)
}
