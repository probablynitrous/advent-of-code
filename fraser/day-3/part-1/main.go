package main

import (
	"bufio"
	"os"
)

type square struct {
	tree bool
}

func newSquare(squareRune rune) square {
	return square{tree: string(squareRune) == "#"}
}

func expand(grid [][]square) [][]square {

	// For every line
	for index, line := range grid {
		// Append another instance of the line to itself
		grid[index] = append(line, line...)
	}

	return grid
}

func main() {
	const dx = 3
	const dy = 1

	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	defer file.Close()

	var grid = [][]square{}

	scanner := bufio.NewScanner(file)

	// For each line
	for scanner.Scan() {
		runes := []rune(scanner.Text())
		var line = []square{}

		// For each character in the string
		for _, rune := range runes {
			line = append(line, newSquare(rune))
		}
		grid = append(grid, line)
	}

	// Count hit trees
	counter := 0

	// Count x index
	x := 0

	// Traverse the map
	for y := 0; y < len(grid); y += dy {
		// If we are off the right hand side of the map
		if !(x < len(grid[y])) {
			grid = expand(grid)
		}

		if grid[y][x].tree {
			counter++
		}
		x += dx
	}

	println("Hit ", counter, " trees")
}
