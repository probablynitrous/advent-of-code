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

	// Init to a value that will "never occur"
	// Yes this is bad practice
	var previous int = -1

	// Count the result
	var result int = 0

	// Parse lines
	for scanner.Scan() {
		line := scanner.Text()
		// Line to int
		depth, _ := strconv.Atoi(line)

		// If this is the first line then move on
		if previous == -1 {
			previous = depth
			continue
		}

		// If we are deeper then increment result
		if previous < depth {
			result = result + 1
		}

		// Update previous
		previous = depth
	}

	println("Result", result)
}
