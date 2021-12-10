package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	data := readFile("input.txt")
	totalOutput := getUniqueCounter(data)
	fmt.Println(totalOutput)
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
	totalOutput := getTotalOutput(segments)
	return totalOutput
}

func splitLines(data []string) [][]string {
	segments := make([][]string, len(data))
	for i, line := range data {
		segment := strings.Split(line, " | ")
		segments[i] = segment
	}

	return segments
}
func existsInSlice(search string, list []string) bool {
	for _, target := range list {
		if target == search {
			return true
		}
	}
	return false
}

func containsAllChar(input string, search string) bool {
	stringSlice := strings.Split(search, "")
	for _, character := range stringSlice {
		if strings.Contains(input, character) == false {
			return false
		}
	}
	return true
}

func findInSlice(list [10]string, search string) int {
	for i, target := range list  {
		if target == search {
			return i
		}
	}
	return len(list)
}

func sortValue(input string) string {
	splitInput := strings.Split(input,"")
	sort.Strings(splitInput)
	return strings.Join(splitInput[:],"")
}

func removeChar(input string, search string) string {
	newString := input
	stringSlice := strings.Split(search, "")
	for _, character := range stringSlice {
		newString = strings.ReplaceAll(newString, character, "")
	}

	return newString
}


func getTotalOutput(segments [][]string) int {
	var totalOutput int
	for _, segment := range segments {
		var uniqueValues []string
		input := strings.Split(segment[0], " ")
		for _, inputValue := range input {
			if existsInSlice(inputValue, uniqueValues) == false {
				uniqueValues = append(uniqueValues, inputValue)
			}
		}
		numbers := setNumbers(uniqueValues)
		totalOutput += getOutputValue(segment, numbers)
	}
	return totalOutput
}

// if theres 5 sides and 3 match that of 7 then its 3
// if theres 5 sides and 2 of the sides match 2 of 4s side after removing the sides that match 1 then its 5
// if theres 5 sides and its neither of the above then its 2

// if theres 6 sides and one of 1s sides is not there then its 6
// if theres 6 sides and all of 4s sides are there then its 9
// if theres 6 sides and ts none of the above then its 0

//#quikmaffs

func setNumbers(uniqueValues []string) [10]string {
	numbers := setKnownValues(uniqueValues)
	
	//Sets unknown values
	for _, value := range uniqueValues {
		value = sortValue(value)
		switch len(value) {
		case 5:
			if containsAllChar(value, numbers[7]) {
				numbers[3] = value
			} else {
				if containsAllChar(value, removeChar(numbers[4], numbers[1])) {
					numbers[5] = value
				} else {
					numbers[2] = value
				}
			}
		case 6:
			if containsAllChar(value, numbers[1]) == false {
				numbers[6] = value
			} else {
				if containsAllChar(value, numbers[4]) {
					numbers[9] = value
				} else {
					numbers[0] = value
				}
			}
		}
	}
	return numbers
}

func setKnownValues(uniqueValues []string) [10]string {
	var knowValues [10]string
	for _, value := range uniqueValues {
		value = sortValue(value)
		switch len(value) {
		case 2:
			knowValues[1] = value
		case 4:
			knowValues[4] = value
		case 7:
			knowValues[8] = value
		case 3:
			knowValues[7] = value
		}
	}
	return knowValues
}

func getOutputValue(segment []string, numbers [10]string) int {
	var endOutputValue string
	output := strings.Split(segment[1], " ")
	for _, outputValue := range output {
		sortedValue := sortValue(outputValue)
		endOutputValue += strconv.Itoa(findInSlice(numbers, sortedValue))
	}
	outputInt, _ := strconv.Atoi(endOutputValue)
	return outputInt
}
