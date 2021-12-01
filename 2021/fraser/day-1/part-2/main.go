package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func main() {

	if len(os.Args) < 2 {
		log.Fatal("Missing required arg filename")
	}

	inputFileName := os.Args[1]

	// Assume we can always open the file
	file, _ := os.Open(inputFileName)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	// Count the result
	var result int = 0

	var window = []int{-1, -1, -1}

	// Parse lines
	for scanner.Scan() {
		line := scanner.Text()
		// Line to int
		depth, _ := strconv.Atoi(line)

		// We need two full windows to compare
		if !windowIsFull(window) {
			// Create nexdt and continue
			window = createNextWindow(window, depth)
			continue
		}

		previousWindowSum := sumWindow(window)
		window = createNextWindow(window, depth)
		windowSum := sumWindow(window)

		if previousWindowSum < windowSum {
			result = result + 1
		}
	}

	println("Result", result)
}

func windowIsFull(window []int) bool {
	for _, val := range window {
		if val < 0 {
			return false
		}
	}
	return true
}

func createNextWindow(window []int, next int) []int {
	return []int{window[1], window[2], next}
}

func sumWindow(window []int) int {
	var total int
	for _, value := range window {
		total = total + value
	}
	return total
}
