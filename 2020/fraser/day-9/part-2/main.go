package main

import (
	"bufio"
	"errors"
	"log"
	"os"
	"sort"
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

	var target int

	for index := preamble + 1; index < len(numbers); index++ {
		if !canMake(numbers[index], numbers[index-preamble:index]) {
			log.Printf("Failed to make %v", numbers[index])
			target = numbers[index]
			break
		}
	}

	numbers, err := findContiguous(target)

	if err != nil {
		log.Fatal(err.Error())
	}

	sort.Ints(numbers)

	log.Printf("Result %v", numbers[0]+numbers[len(numbers)-1])
}

func findContiguous(target int) ([]int, error) {
	println("Finding")
	for start := 0; start < len(numbers); start++ {
		for end := start; end < len(numbers); end++ {
			subset := numbers[start:end]
			sum := sum(subset)
			if sum == target {
				return numbers[start:end], nil
			}
		}
	}
	return nil, errors.New("Failed to find result")
}

func sum(array []int) int {
	result := 0
	for _, v := range array {
		result += v
	}
	return result
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
