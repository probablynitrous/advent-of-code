package main

import (
	"bufio"
	"os"
	"sync"
)

const threads = 8

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	passes := []string{}

	// For each line
	for scanner.Scan() {
		passes = append(passes, scanner.Text())
	}

	file.Close()

	highest := 0

	lock := sync.RWMutex{}

	size := len(passes) / threads

	wg := sync.WaitGroup{}
	wg.Add(threads)
	for i := 0; i < threads; i++ {
		go func(passes []string) {
			defer wg.Done()
			for _, pass := range passes {
				id := getID(pass)
				lock.Lock()
				if highest < id {
					highest = id
				}
				lock.Unlock()
			}
		}(passes[i*size : (i+1)*size])
	}
	wg.Wait()
	println("Answer", highest)
}

func getID(pass string) int {
	runes := []rune(pass)
	rowLower := 0
	rowUpper := 127
	columnLower := 0
	columnUpper := 7
	var row int
	var column int

	for _, rune := range runes {
		character := string(rune)
		switch character {
		case "F":
			{
				if (rowUpper - rowLower) == 1 {
					row = rowLower
				}
				rowUpper = (rowLower + rowUpper) / 2
			}
		case "B":
			{
				if (rowUpper - rowLower) == 1 {
					row = rowUpper
				}
				rowLower = ((rowLower + rowUpper) / 2) + 1
			}

		case "L":
			{
				if (columnUpper - columnLower) == 1 {
					column = columnLower
				}
				columnUpper = (columnLower + columnUpper) / 2
			}
		case "R":
			{
				if (columnUpper - columnLower) == 1 {
					column = columnUpper
				}
				columnLower = ((columnLower + columnUpper) / 2) + 1
			}
		}
	}
	return (row * 8) + column
}
