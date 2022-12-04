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

	sack := ""
	firstTotal := 0
	groupTotal := 0

	sets := []map[rune]struct{}{{}, {}}

	sackRune := ' '
	groupRune := ' '

	i := 0

	for scanner.Scan() {
		set := map[rune]struct{}{}
		seen := map[rune]struct{}{}

		sack = scanner.Text()

		for j, r := range sack {
			if j < len(sack)/2 {
				seen[r] = struct{}{}
			} else if _, ok := seen[r]; ok {
				sackRune = r
			}

			if i%3 == 2 {
				_, ok1 := sets[0][r]
				_, ok2 := sets[1][r]

				if ok1 && ok2 {
					groupRune = r
				}
			}

			set[r] = struct{}{}
		}

		switch i % 3 {
		case 0:
			sets[0] = set
		case 1:
			sets[1] = set
		case 2:
			groupTotal += getPrio(groupRune)
		}

		firstTotal += getPrio(sackRune)

		i++
	}

	fmt.Println("Part 1: ", firstTotal)
	fmt.Println("Part 2: ", groupTotal)
}

func getPrio(r rune) int {
	prio := 0

	// is lower
	if r > 96 {
		prio = int(r) - 96
	} else {
		prio = int(r) - 64 + 26
	}

	return prio
}
