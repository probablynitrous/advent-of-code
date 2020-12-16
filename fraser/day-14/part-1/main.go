package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

type instruction struct {
	address string
	value   int
}

// 14087492730470
//Too high

func main() {
	// Assume we can always open the file
	file, _ := os.Open(os.Args[1])
	scanner := bufio.NewScanner(file)

	lineIndex := 0

	var mask []string

	instructions := []instruction{}

	memory := make(map[string]int)

	maskRegex := regexp.MustCompile("mask = ([X0-9]+)")

	// For each line
	for scanner.Scan() {
		defer func() { file.Close() }()
		line := scanner.Text()

		// Line is a mask
		if maskRegex.MatchString(line) {
			// Start with blank mask
			mask = []string{}
			maskString := maskRegex.FindStringSubmatch(line)[1]

			for _, rune := range []rune(maskString) {
				mask = append(mask, string(rune))
			}

			lineIndex++
			continue
		}

		matches := regexp.MustCompile("mem\\[([0-9]+)\\] = ([0-9]+)").FindStringSubmatch(line)

		if len(matches) < 1 {
			log.Fatalf("Failed to build instruction from %v", line)
		}

		value, _ := strconv.Atoi(matches[2])
		instruction := instruction{address: matches[1], value: value}
		instructions = append(instructions, instruction)
	}

	// Process instructions
	for _, instruction := range instructions {
		bits := []rune(fmt.Sprintf("%0*b", len(mask), instruction.value))

		for bitIndex, maskBit := range mask {
			switch maskBit {
			case "X":
				{
					continue
				}
			default:
				{
					bits[bitIndex] = []rune(maskBit)[0]
				}
			}
		}

		maskedInt, _ := strconv.ParseInt(string(bits), 2, len(bits))
		memory[instruction.address] = int(maskedInt)
	}

	result := 0

	for _, value := range memory {
		result += value
	}

	log.Printf("Result %v", result)
}
