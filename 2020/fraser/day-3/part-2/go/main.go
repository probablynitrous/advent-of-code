package main

import (
	"bufio"
	"os"
)

type square struct {
	tree bool
}

type slope struct {
	dx int
	dy int
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

// Count the trees hit for a specific slope
func countTrees(result chan int, slope slope, grid [][]square) {
	// Count hit trees
	counter := 0

	// Count x index
	x := 0

	// Traverse the map
	for y := 0; y < len(grid); y += slope.dy {
		// If we are off the right hand side of the map
		if !(x < len(grid[y])) {
			grid = expand(grid)
		}

		if grid[y][x].tree {
			counter++
		}
		x += slope.dx
	}

	result <- counter
}

func main() {
	var slopes = []slope{
		{dx: 1, dy: 1},
		{dx: 3, dy: 1},
		{dx: 5, dy: 1},
		{dx: 7, dy: 1},
		{dx: 1, dy: 2},
	}

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

	counter := 1
	finished := 0

	results := make(chan int)

	for _, slope := range slopes {
		go countTrees(results, slope, grid)
	}

	for {
		if finished == len(slopes) {
			break
		}
		counter = counter * <-results
		finished++
	}

	println("Answer is", counter)
}
