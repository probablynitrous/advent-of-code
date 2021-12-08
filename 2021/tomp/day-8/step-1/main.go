package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type UniqueValue struct {
	one   int
	four  int
	seven int
	eight int
}

func main() {
	data := readFile("input.txt")
	uniqueCount := getUniqueCounter(data)
	fmt.Println(uniqueCount)
}

func readFile(fileName string) []string {
	data, _ := os.Open(fileName)
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}
	return dataLines
}

func getUniqueCounter(data []string) int {
	segments := splitLines(data)
	uniqueValues := findUniqueValues(segments)
	uniqueCounter := uniqueValues.one + uniqueValues.four + uniqueValues.seven + uniqueValues.eight
	return uniqueCounter
}

func splitLines(data []string) [][]string {
	segments := make([][]string, len(data))
	for i, line := range data {
		segment := strings.Split(line, " | ")
		segments[i] = segment
	}

	return segments
}

func findUniqueValues(segments [][]string) UniqueValue {
	var uniqueValues UniqueValue
	for _, segment := range segments {
		output := strings.Split(segment[1], " ")
		for _, outputValue := range output {
			switch len(outputValue) {
			case 2:
				uniqueValues.one += 1
			case 4:
				uniqueValues.four += 1
			case 3:
				uniqueValues.seven += 1
			case 7:
				uniqueValues.eight += 1
			}
		}
	}

	return uniqueValues
}
