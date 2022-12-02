package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	horiz := 0
	depth := 0

	aim := 0
	horiz2 := 0
	depth2 := 0

	num := 0

	vals := make([]string, 2)

	line := ""
	for scanner.Scan() {
		line = scanner.Text()

		vals = strings.Split(line, " ")
		num, _ = strconv.Atoi(vals[1])

		switch vals[0] {
		case "forward":
			horiz += num
			horiz2 += num
			depth2 += aim * num
		case "down":
			depth += num
			aim += num
		case "up":
			depth -= num
			aim -= num
		}
	}

	fmt.Println("first part", horiz*depth)
	fmt.Println("second part", horiz2*depth2)
}
