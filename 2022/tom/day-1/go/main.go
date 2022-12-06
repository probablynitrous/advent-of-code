package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"flag"
)


func main() {
	data := getInput()
	step1 := step1(data)
	fmt.Println("Step 1: ",step1)
	step2 := step2(data)
	fmt.Println("Step 2: ",step2)
}

func getInput() []string {
	testArg := flag.Bool("test", false, "")
	flag.Parse()
	var test bool = bool(*testArg)

	if test {
		return readFile("../test.txt")
	} else {
		return readFile("../input.txt")
	}
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

func step1(data []string) int {
	var topCalories int = 0
	var currentCalories int = 0
	for _,line := range data {
		if (line == "") {
			if topCalories < currentCalories {
				topCalories = currentCalories
			}
			currentCalories = 0
		} else {
			calories,_ := strconv.Atoi(line)
			currentCalories += calories	
		}
	
	} 
	
	if topCalories < currentCalories {
		topCalories = currentCalories
	}
	return topCalories
}

func step2(data []string) int {
	topCalories := []int{0,0,0}
	var currentCalories int = 0
	for _,line := range data {
		if (line == "") {
			topCalories = updateTopCalories(topCalories,currentCalories)
			currentCalories = 0
		} else {
			calories,_ := strconv.Atoi(line)
			currentCalories += calories	
		}
			sort.Ints(topCalories)
	} 
	
	topCalories = updateTopCalories(topCalories,currentCalories)

	var total int = 0
	for _,calories := range topCalories{
		total += calories
	}
	return total
}

func updateTopCalories(topCalories []int, currentCalories int) []int {
	if currentCalories > topCalories[0] {
		topCalories = append(topCalories, currentCalories)
		topCalories = topCalories[1:]
	}

	return topCalories
}