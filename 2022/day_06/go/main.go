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
	msgSize := 14

	s := input[:packetSize]
	s2 := input[:msgSize]
	packetFound := false
	msgFound := false

	packetMark := -1
	msgMark := -1
	for i := packetSize + 1; i < len(input); i++ {
		if packetFound && msgFound {
			break
		}
		if !packetFound {
			s = input[i-packetSize : i]
			if checkWindow(s, packetSize) {
				packetMark = i
				packetFound = true
			}
		}
		if !msgFound && i > msgSize {
			s2 = input[i-msgSize : i]
			if checkWindow(s2, msgSize) {
				msgMark = i
				msgFound = true
			}
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
