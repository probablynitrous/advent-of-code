package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Brackets struct {
	curly  string
	round  string
	square string
	angle  string
}

var pairings = map[string]string{"{": "}", "[": "]", "(": ")", "<": ">"}
var multipliers = map[string]int{"}": 1197, "]": 57, ")": 3, ">": 25137}
var corruptChars []string
var currOpen []string

func main() {
	data := readFile("input.txt")
	errorScore := getErrorScore(data)
	fmt.Println(errorScore)
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

func getErrorScore(data []string) int {
	for _, line := range data {
		checkLine(line)
	}
	var illegalCount int
	for bracket,multiplier := range multipliers {
		illegalCount += count(bracket, corruptChars) * multiplier
	}

	return illegalCount

}

func checkLine(line string) {
	t := strings.Split(line, "")
	currOpen = nil
	var open []string
	for _, value := range t {
		open = append(open, value)
		if opener(value) {
			currOpen = append(currOpen, value)
		}
		if closer(value) {
			if !checkForOpener(value) {
				break
			}
		}

	}
}

func opener(search string) bool {
	for opener := range pairings {
		if opener == search {
			return true
		}
	}
	return false
}

func closer(search string) bool {
	for _, closer := range pairings {
		if closer == search {
			return true
		}
	}
	return false
}

func find(search string, list []string) int {
	for i, target := range list {
		if target == search {
			return i
		}
	}
	return len(list)
}

func getMultiplier(search string) int {
	for i, multiplier := range multipliers {
		if i == search {
			return multiplier
		}
	}
	return 0
}

func count(search string, list []string) int {
	var counter int
	for _, target := range list {
		if target == search {
			counter += 1
		}
	}
	return counter
}

func checkForOpener(search string) bool {
	for opener, closer := range pairings {
		if closer == search {
			if currOpen[len(currOpen)-1] == opener {
				currOpen = pop(currOpen)
				return true
			} else {
				corruptChars = append(corruptChars, search)
				return false
			}
		}
	}
	return false

}

func pop(inputSlice []string) []string {
	var outputSlice []string
	for i := 0; i < len(inputSlice)-1; i++ {
		outputSlice = append(outputSlice, inputSlice[i])
	}
	
	return outputSlice
}
