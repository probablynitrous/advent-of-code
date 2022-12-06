package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"regexp"
	"flag"
)

type Pair struct {
	lower int
	upper int
}

func main() {
	testArg := flag.Bool("test", false, "")
	flag.Parse()
	var test bool = bool(*testArg)

	var data []string
	if test {
		data = readFile("../test.txt")
	} else {
		data = readFile("../input.txt")
	}
	
	step1 := step1(data)
	fmt.Println("Step 1: ",step1)
	step2 := step2(data)
	fmt.Println("Step 2: ",step2)
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
	var containedPairs int
	for _,line := range data {
		var pair1 Pair
		var pair2 Pair
		r := regexp.MustCompile("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)")

		pairs := r.FindAllStringSubmatch(line,-1)[0]
		
		pair1.lower,_ = strconv.Atoi(pairs[1])
		pair1.upper,_ = strconv.Atoi(pairs[2])
		pair2.lower,_ = strconv.Atoi(pairs[3])
		pair2.upper,_ = strconv.Atoi(pairs[4])

		if (checkBetween(pair1.lower, pair2)) && 
			 (checkBetween(pair1.upper, pair2)) {
				containedPairs += 1
				continue
		}
		
		if (checkBetween(pair2.lower, pair1)) && 
			 (checkBetween(pair2.upper, pair1)) {
				containedPairs += 1
				continue
		}
	}
	return containedPairs
}

func step2(data []string) int {
	var containedPairs int
	for _,line := range data {
		var pair1 Pair
		var pair2 Pair
		r := regexp.MustCompile("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)")

		pairs := r.FindAllStringSubmatch(line,-1)[0]
		
		pair1.lower,_ = strconv.Atoi(pairs[1])
		pair1.upper,_ = strconv.Atoi(pairs[2])
		pair2.lower,_ = strconv.Atoi(pairs[3])
		pair2.upper,_ = strconv.Atoi(pairs[4])

		if (checkBetween(pair1.lower, pair2)) || 
			 (checkBetween(pair1.upper, pair2)) {
				containedPairs += 1
				continue
		}
		
		if (checkBetween(pair2.lower, pair1)) || 
			 (checkBetween(pair2.upper, pair1)) {
				containedPairs += 1
				continue
		}
	}
	return containedPairs
}

func checkBetween(i int, checkRange Pair) bool {
	if (i >= checkRange.lower) && (i <= checkRange.upper) {
		return true
	} else {
		return false
	}
}