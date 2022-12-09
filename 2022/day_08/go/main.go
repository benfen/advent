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
	fmt.Println("Part 2:", getScenicScore(forest))
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

func getScenicScore(forest [][]int) int {
	bestScore := 0

	for i := 1; i < len(forest)-1; i++ {
		for j := 1; j < len(forest[0])-1; j++ {
			height := forest[i][j]
			left, right, up, down := 0, 0, 0, 0
			for x := j + 1; x < len(forest[0]); x++ {
				right++
				if forest[i][x] >= height {
					break
				}
			}

			for x := j - 1; x > -1; x-- {
				left++
				if forest[i][x] >= height {
					break
				}
			}

			for y := i + 1; y < len(forest); y++ {
				up++
				if forest[y][j] >= height {
					break
				}
			}

			for y := i - 1; y > -1; y-- {
				down++
				if forest[y][j] >= height {
					break
				}
			}
			score := left * right * up * down
			if score > bestScore {
				bestScore = score
			}
		}
	}

	return bestScore
}
