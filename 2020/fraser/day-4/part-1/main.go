package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var prefixes = []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

func valid(line string) bool {
	for _, prefix := range prefixes {
		if !strings.Contains(line, prefix) {
			println("Line", line, "does not contain", prefix)
			return false
		}
	}
	return true
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	valids := 0
	passportString := ""

	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		passportString = fmt.Sprintf("%v %v", passportString, line)
		if line == "" {
			println("passportString", passportString)
			if valid(passportString) {
				valids++
			}
			passportString = ""
		}
	}

	println("There are", valids, "invalid passports")
}
