package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	forest := [][]int{}

	line := ""
	for scanner.Scan() {
		line = scanner.Text()

		trees := []int{}

		for i := range line {
			num, _ := strconv.Atoi(line[i : i+1])
			trees = append(trees, num)
		}

		forest = append(forest, trees)
	}

	fmt.Println("Part 1:", getVisibleTrees(forest))
}

func getVisibleTrees(forest [][]int) int {
	trees := 0
	for i := 0; i < len(forest); i++ {
		for j := 0; j < len(forest[0]); j++ {
			height := forest[i][j]

			stillVisible := true
			for x := j + 1; x < len(forest[0]); x++ {
				if forest[i][x] >= height {
					stillVisible = false
					break
				}
			}
			if stillVisible {
				trees++
				continue
			}

			stillVisible = true
			for x := j - 1; x > -1; x-- {
				if forest[i][x] >= height {
					stillVisible = false
					break
				}
			}

			if stillVisible {
				trees++
				continue
			}

			stillVisible = true
			for y := i + 1; y < len(forest); y++ {
				if forest[y][j] >= height {
					stillVisible = false
					break
				}
			}
			if stillVisible {
				trees++
				continue
			}

			stillVisible = true
			for y := i - 1; y > -1; y-- {
				if forest[y][j] >= height {
					stillVisible = false
					break
				}
			}

			if stillVisible {
				trees++
				continue
			}
		}
	}

	return trees
}
