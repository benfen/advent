package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Node struct {
	size     int
	name     string
	children []*Node
	parent   *Node
	nodeType string
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	var root *Node
	var curr *Node

	line := ""
	for scanner.Scan() {
		line = scanner.Text()

		if line == "$ ls" {
			continue
		}

		if strings.HasPrefix(line, "$ cd") {
			name := strings.Split(line, "$ cd ")[1]

			if root == nil {
				root = &Node{
					name:     name,
					nodeType: "dir",
				}
			}

			if name == ".." {
				curr = curr.parent
			} else if name == "/" {
				curr = root
			} else {
				for _, node := range curr.children {
					if node.name == name {
						curr = node
					}
				}
			}
			continue
		}

		dirOrFile := strings.Split(line, " ")
		newNode := Node{
			name:     dirOrFile[1],
			parent:   curr,
			nodeType: "dir",
		}
		if dirOrFile[0] != "dir" {
			size, _ := strconv.Atoi(dirOrFile[0])
			newNode.size = size
			newNode.nodeType = "file"
		}

		curr.children = append(curr.children, &newNode)
	}

	fmt.Println("Part 1:", calculateSizes(root, 0))

	totalSpace := 70000000
	spaceRequired := 30000000
	needToDelete := spaceRequired - (totalSpace - root.size)

	smallestDir := getSmallestDirPossible(root, root, needToDelete)

	fmt.Println("Part 2:", smallestDir.size)
}

func calculateSizes(n *Node, totalSum int) int {
	for _, child := range n.children {
		if child.nodeType == "dir" {
			totalSum = calculateSizes(child, totalSum)
		}
		n.size += child.size
	}

	if n.size <= 100000 {
		totalSum += n.size
	}

	return totalSum
}

func getSmallestDirPossible(n *Node, smallestSeen *Node, spaceNeeded int) *Node {
	for _, child := range n.children {
		if child.nodeType == "dir" {
			smallestSeen = getSmallestDirPossible(child, smallestSeen, spaceNeeded)
		}

		if n.size > spaceNeeded && n.size < smallestSeen.size {
			smallestSeen = n
		}
	}
	return smallestSeen
}
