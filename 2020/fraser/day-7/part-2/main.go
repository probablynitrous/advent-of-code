package main

import (
	"bufio"
	"os"
	"regexp"
	"strconv"
)

const threads = 8

var bags = make(map[string]bag)

type bag struct {
	colour       string
	requiredBags []bagPolicy
}

type bagPolicy struct {
	count  int
	colour string
}

// Return the number of bags this bag must contain
func (outer bag) contains() int {
	count := 0

	// Add immediate child bags
	for _, requiredBagPolicy := range outer.requiredBags {
		// Immediate chilc
		count = count + requiredBagPolicy.count
		// Recurse child's children
		count = count + (requiredBagPolicy.count * bags[requiredBagPolicy.colour].contains())

	}
	return count
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		outerRegex := regexp.MustCompile("^([a-z]+ [a-z]+)")
		outerColour := outerRegex.FindStringSubmatch(line)[1]
		outer := bag{colour: outerColour}

		innerRegex := regexp.MustCompile("([0-9]+) ([a-z]+ [a-z]+)")

		inners := innerRegex.FindAllStringSubmatch(line, -1)

		for _, matches := range inners {
			count, _ := strconv.Atoi(matches[1])
			innerBagPolicy := bagPolicy{count: count, colour: matches[2]}
			outer.requiredBags = append(outer.requiredBags, innerBagPolicy)
		}

		bags[outerColour] = outer
	}

	file.Close()

	result := bags["shiny gold"].contains()

	println("Result", result)
}
