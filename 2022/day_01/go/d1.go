package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"strconv"
)

type IntHeap []int

func (h IntHeap) Len() int {
	return len(h)
}

func (h IntHeap) Less(i, j int) bool {
	return h[j] < h[i]
}

func (h IntHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *IntHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	pq := &IntHeap{}
	heap.Init(pq)

	currElf := 0
	val := 0
	line := ""
	for scanner.Scan() {
		line = scanner.Text()
		if line == "" {
			heap.Push(pq, currElf)
			currElf = 0
		} else {
			val, err = strconv.Atoi(line)
			if err != nil {
				fmt.Println(err)
				return
			}

			currElf += val
		}
	}

	total := heap.Pop(pq).(int)

	fmt.Println("Part 1: ", total)

	for i := 0; i < 2; i++ {
		total += heap.Pop(pq).(int)
	}

	fmt.Println("Part 2: ", total)
}
