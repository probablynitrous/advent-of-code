package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

const preamble = 25

var numbers = []int{}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		number, _ := strconv.Atoi(line)
		numbers = append(numbers, number)
	}

	file.Close()

	for index := preamble + 1; index < len(numbers); index++ {
		if !canMake(numbers[index], numbers[index-preamble:index]) {
			log.Fatalf("Failed to make %v", numbers[index])
		}
	}

	log.Println("Finished")
}

func canMake(targetNumber int, numbers []int) bool {
	for _, numberOne := range numbers {
		for _, numberTwo := range numbers {
			if (numberOne + numberTwo) == targetNumber {
				return true
			}
		}
	}
	return false
}
