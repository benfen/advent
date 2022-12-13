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

	cycle := 0
	reg := 1
	signalStrength := 0
	ascii := []rune{}

	increment := func(val int) {
		cycle++
		if (cycle % 40) == 20 {
			signalStrength += cycle * reg
		}

		spriteCheck := (cycle-1)%40 + 1
		if spriteCheck >= reg && spriteCheck <= reg+2 {
			ascii = append(ascii, '#')
		} else {
			ascii = append(ascii, '.')
		}

		reg += val
	}

	input := ""
	for scanner.Scan() {
		input = scanner.Text()

		if input != "noop" {
			in := strings.Split(input, " ")
			num, _ := strconv.Atoi(in[1])

			increment(0)
			increment(num)

		} else {
			increment(0)
		}
	}

	fmt.Println("Part 1:", signalStrength)

	for i, sprite := range ascii {
		if i%40 == 0 {
			fmt.Println()
		}

		fmt.Printf("%s", string(sprite))
	}
}
