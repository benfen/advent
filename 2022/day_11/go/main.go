package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	START = "Starting items: "
	OP    = "Operation: "
	TEST  = "Test: divisible by "
	PASS  = "If true: throw to monkey "
	FAIL  = "If false: throw to monkey "
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	monkeys := []*Monkey{}
	monkeys2 := []*Monkey{}

	bigMod := 1

	line := ""
	var monkey *Monkey
	var monkey2 *Monkey
	for scanner.Scan() {
		line = scanner.Text()

		if strings.HasPrefix(line, "Monkey") {
			monkey = NewMonkey()
			monkeys = append(monkeys, monkey)

			monkey2 = NewMonkey()
			monkeys2 = append(monkeys2, monkey2)
		}

		if strings.Contains(line, START) {
			s := strings.Split(line, START)
			monkey.AddItems(s[1])
			monkey2.AddItems(s[1])
		}

		if strings.Contains(line, OP) {
			s := strings.Split(line, OP)
			monkey.AddOp(s[1])
			monkey2.AddOp(s[1])
		}

		if strings.Contains(line, TEST) {
			s := strings.Split(line, TEST)
			monkey.AddCheck(s[1])
			bigMod *= monkey2.AddCheck(s[1])
		}
		if strings.Contains(line, PASS) {
			s := strings.Split(line, PASS)
			monkey.AddPass(s[1])
			monkey2.AddPass(s[1])
		}
		if strings.Contains(line, FAIL) {
			s := strings.Split(line, FAIL)
			monkey.AddFail(s[1])
			monkey2.AddFail(s[1])
		}
	}

	for i := 0; i < 20; i++ {
		PlayRound(monkeys)
	}

	for i := 0; i < 10000; i++ {
		PlayRound2(monkeys2, bigMod)
	}

	pq := &Heap{}
	heap.Init(pq)

	for _, m := range monkeys {
		heap.Push(pq, m)
	}

	pq2 := &Heap{}
	heap.Init(pq2)

	for _, m := range monkeys2 {
		heap.Push(pq2, m)
	}

	monkeyBusiness := heap.Pop(pq).(*Monkey).Inspections * heap.Pop(pq).(*Monkey).Inspections
	monkeyBusiness2 := heap.Pop(pq2).(*Monkey).Inspections * heap.Pop(pq2).(*Monkey).Inspections

	fmt.Println("Part 1", monkeyBusiness)
	fmt.Println("Part 2", monkeyBusiness2)
}

type Monkey struct {
	Items       []int
	Op          func(int) int
	Check       func(int) bool
	Pass        func(int, []*Monkey)
	Fail        func(int, []*Monkey)
	Inspections int
}

func NewMonkey() *Monkey {
	return &Monkey{
		Items: []int{},
	}
}

func (m *Monkey) AddItems(itemsS string) {
	numS := strings.Split(itemsS, ", ")
	for _, num := range numS {
		item, _ := strconv.Atoi(num)
		m.Items = append(m.Items, item)
	}
}

func (m *Monkey) AddOp(s string) {
	ss := strings.Split(s, "= ")

	val := 0

	self := false

	if strings.Contains(ss[1], "+") {
		vals := strings.Split(ss[1], "+ ")
		if vals[1] == "old" {
			self = true
		}

		if !self {
			val, _ = strconv.Atoi(vals[1])
		}

		m.Op = func(num int) int {
			if self {
				return num + num
			}
			return num + val
		}
	}

	if strings.Contains(ss[1], "*") {
		vals := strings.Split(ss[1], "* ")
		if vals[1] == "old" {
			self = true
		}

		if !self {
			val, _ = strconv.Atoi(vals[1])
		}

		m.Op = func(num int) int {
			if self {
				return num * num
			}
			return num * val
		}
	}

}

func (m *Monkey) AddCheck(s string) int {
	val, _ := strconv.Atoi(s)
	m.Check = func(num int) bool {
		return num%val == 0
	}

	return val
}

func (m *Monkey) AddPass(s string) {
	val, _ := strconv.Atoi(s)
	m.Pass = func(num int, monkeys []*Monkey) {
		monkeys[val].Items = append(monkeys[val].Items, num)
	}
}

func (m *Monkey) AddFail(s string) {
	val, _ := strconv.Atoi(s)
	m.Fail = func(num int, monkeys []*Monkey) {
		monkeys[val].Items = append(monkeys[val].Items, num)
	}
}

func (m *Monkey) TakeTurn(monkeys []*Monkey) {
	for _, item := range m.Items {
		m.Inspections++

		item = m.Op(item)
		item /= 3

		if m.Check(item) {
			m.Pass(item, monkeys)
		} else {
			m.Fail(item, monkeys)
		}
	}

	m.Items = []int{}
}

func (m *Monkey) TakeBigTurn(monkeys []*Monkey, mod int) {
	for _, item := range m.Items {
		m.Inspections++

		item = m.Op(item) % mod

		if m.Check(item) {
			m.Pass(item, monkeys)
		} else {
			m.Fail(item, monkeys)
		}
	}

	m.Items = []int{}
}

func PlayRound(monkeys []*Monkey) {
	for i := 0; i < len(monkeys); i++ {
		monkeys[i].TakeTurn(monkeys)
	}
}

func PlayRound2(monkeys2 []*Monkey, mod int) {
	for i := 0; i < len(monkeys2); i++ {
		monkeys2[i].TakeBigTurn(monkeys2, mod)
	}
}

type Heap []*Monkey

func (h Heap) Len() int {
	return len(h)
}

func (h Heap) Less(i, j int) bool {
	return h[j].Inspections < h[i].Inspections
}

func (h Heap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *Heap) Push(x any) {
	*h = append(*h, x.(*Monkey))
}

func (h *Heap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}
