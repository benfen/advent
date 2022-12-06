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

	scanner.Scan()
	input := scanner.Text()

	packetSize := 4

	s := input[:packetSize]

	packetMark := -1
	for i := packetSize + 1; i < len(input); i++ {
		s = input[i-packetSize : i]
		if checkWindow(s, packetSize) {
			packetMark = i
			break
		}
	}

	msgSize := 14
	msgMark := -1
	s = input[:msgSize]
	for i := msgSize + 1; i < len(input); i++ {
		s = input[i-msgSize : i]
		if checkWindow(s, msgSize) {
			msgMark = i
			break
		}
	}

	fmt.Println(packetMark)
	fmt.Println(msgMark)
}

func checkWindow(s string, size int) bool {
	for i := 0; i < size; i++ {
		for j := i + 1; j < size; j++ {
			if s[i] == s[j] {
				return false
			}
		}
	}

	return true
}
