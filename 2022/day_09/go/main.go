package main

import (
	"bufio"
	"fmt"
	"math"
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

	head := Pt{X: 0, Y: 0}
	visited := map[Pt]struct{}{head: {}}
	longVisited := map[Pt]struct{}{head: {}}

	snake := [10]*Pt{}

	for i := 0; i < 10; i++ {
		snake[i] = &Pt{X: 0, Y: 0}
	}

	var input []string
	var pt Pt
	for scanner.Scan() {
		input = strings.Split(scanner.Text(), " ")
		distance, _ := strconv.Atoi(input[1])
		pt = buildPoint(input[0])

		for i := 0; i < distance; i++ {
			snake[0].Add(pt)
			for j := 1; j < len(snake); j++ {
				moved := movePointsIfNecessary(snake, j, pt, visited, longVisited)
				if !moved {
					break
				}
			}
		}
	}

	fmt.Println("Part 1:", len(visited))
	fmt.Println("Part 2:", len(longVisited))
}

func movePointsIfNecessary(snake [10]*Pt, currIndex int, pt Pt, visited, longVisited map[Pt]struct{}) bool {
	head := snake[currIndex-1]
	tail := snake[currIndex]
	distancePt := tail.Distance(*head)

	// need to move tail to get closer
	if !distancePt.IsReasonable() {
		tail.ApplyMove(distancePt)

		if currIndex == 1 {
			visited[*tail] = struct{}{}
		}
		if currIndex == 9 {
			longVisited[*tail] = struct{}{}
		}
		return true
	}
	return false
}

type Pt struct {
	X int
	Y int
}

func buildPoint(dir string) Pt {
	var pt Pt
	switch dir {
	case "L":
		pt = Pt{
			X: -1,
			Y: 0,
		}
	case "R":
		pt = Pt{
			X: 1,
			Y: 0,
		}
	case "U":
		pt = Pt{
			X: 0,
			Y: 1,
		}
	case "D":
		pt = Pt{
			X: 0,
			Y: -1,
		}
	}

	return pt
}

func (p *Pt) Add(add Pt) {
	p.X += add.X
	p.Y += add.Y
}

// ApplyMove is a special add that gets the distance back to a reasonable space
// i.e. head and tail will be touching again, or distance is no more than 1 in
// any direction
func (tail *Pt) ApplyMove(distance Pt) {
	if distance.X > 0 {
		tail.X++
	} else if distance.X < 0 {
		tail.X--
	}

	if distance.Y > 0 {
		tail.Y++
	} else if distance.Y < 0 {
		tail.Y--
	}
}

// Distance pt between the head and tail
func (tail Pt) Distance(head Pt) Pt {
	needsLeft := 1
	needsDown := 1
	if head.X < tail.X {
		needsLeft = -1
	}
	if head.Y < tail.Y {
		needsDown = -1
	}
	return Pt{
		X: int(math.Abs(float64(tail.X-head.X))) * needsLeft,
		Y: int(math.Abs(float64(tail.Y-head.Y))) * needsDown,
	}
}

// IsReasonable if the tail doesn't need to move yet
func (p Pt) IsReasonable() bool {
	return int(math.Abs(float64(p.X))) < 2 && int(math.Abs(float64(p.Y))) < 2
}
