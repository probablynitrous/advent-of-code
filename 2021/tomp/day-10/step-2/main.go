package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

type Brackets struct {
	curly  string
	round  string
	square string
	angle  string
}

var pairings = map[string]string{"{": "}", "[": "]", "(": ")", "<": ">"}
var multipliers = map[string]int{"}": 3, "]": 2, ")": 1, ">": 4}
var remainders [][]string
var currOpen []string

func main() {
	data := readFile("input.txt")
	errorScore := getCompletionScore(data)
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

func getCompletionScore(data []string) int {
	for _, line := range data {
		if checkLine(line) {
			remainders = append(remainders, getRemainders())
		}
	}	
	var scores []int
	for _,remainder := range remainders {
		scores = append(scores, lineScore(remainder))
	}
	
	return findMedian(scores)

}

func lineScore(remainder []string)int {
	var completionScore int
	for _,char := range remainder {
		completionScore *= 5
		completionScore += getMultiplier(char)
	}

	return completionScore
}

func checkLine(line string) bool{
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
				return false
			}
		}

	}
	return true
}

func getRemainders()[]string {
	var remainder = make([]string, len(currOpen))
	i := len(currOpen)
	for _,opener := range currOpen {
		i--
		remainder[i] = pairings[opener]
	}
	return remainder
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

func findMedian(scores []int)int {
	sort.Ints(scores)
	midPoint := len(scores) / 2
	convergencePoint := scores[midPoint]
	return convergencePoint
}