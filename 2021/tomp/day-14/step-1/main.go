package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strings"
)

type Mappings map[string]string
type Pairs map[string]int
var letterCounter = make(map[string]int)

func main() {
	data := readFile("input.txt")
	elementDiff := getElementDiff(data)
	fmt.Println(elementDiff)
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

func getElementDiff(data []string) int {
	pairs := getOGpairs(data)
	mappings := findMappings(data)
	for i := 0; i < 10 ; i++ {
		pairs = growString(pairs, mappings)
	}
	return getCounterDiff()
}

func getOGpairs(data []string) Pairs {
	pairs := make(Pairs)
	firstLine := strings.Split(data[0], "")
	for i := 0; i < len(firstLine)-1; i++ {
		pairs[firstLine[i]+firstLine[i+1]] += 1
	}

	for _,value := range firstLine {
		letterCounter[value] += 1
	}

	return pairs
}

func findMappings(data []string) Mappings {
	mappings := make(Mappings)
	for _, line := range data {
		regex, _ := regexp.Compile("[A-Z]+ -> [A-Z]")
		if regex.MatchString(line) {
			regex, _ := regexp.Compile("[A-Z]+")
			values := regex.FindAllString(line, 2)
			mappings[values[0]] = values[1]
		}
	}
	return mappings
}

func growString(pairs Pairs, mapping Mappings) Pairs {
	newPairs := make(Pairs)
	for pair, count := range pairs {
		splitPair := strings.Split(pair, "")
		newValue := mapping[pair]
		newPairs[splitPair[0]+newValue] += count
		newPairs[newValue+splitPair[1]] += count
		letterCounter[newValue] += count
	}
	return newPairs
}

func getCounterDiff() int {
	var lowest int
	var highest int
	for _,value := range letterCounter {
		if lowest == 0 || value < lowest {
			lowest = value
		}
		if highest < value {
			highest = value
		}
	}
	return highest - lowest
}