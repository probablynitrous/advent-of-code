package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
)

const threads = 8

var bags = make(map[string]bag)

type bag struct {
	colour         string
	allowedColours []string
}

func (outer bag) canContain(colour string) bool {
	allowed := false

	for _, allowedColour := range outer.allowedColours {
		if allowedColour == colour {
			return true
		}
		if bags[allowedColour].canContain(colour) {
			allowed = true
		}
	}

	// If it has no inner bags then it cannot contain it
	return allowed
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

		innerRegex := regexp.MustCompile("[0-9]+ ([a-z]+ [a-z]+)")

		inners := innerRegex.FindAllStringSubmatch(line, -1)

		for _, matches := range inners {
			innerColour := matches[1]
			outer.allowedColours = append(outer.allowedColours, innerColour)
		}

		bags[outerColour] = outer
	}

	file.Close()

	log.Printf("Have %v bags\n", len(bags))

	result := 0

	// For each bag, if the bag can contain a
	for _, bag := range bags {
		if bag.canContain("shiny gold") {
			result++
		}
	}

	println("Result", result)
}
