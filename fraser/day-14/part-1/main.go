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
// 8568007943024
// Wrong

const maskLength = 36

func main() {
	// Assume we can always open the file
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatalf("Failed to open file")
	}
	scanner := bufio.NewScanner(file)

	var mask string

	memory := make(map[string]int)

	maskRegex := regexp.MustCompile("mask = ([X0-9]+)")

	// For each line
	for scanner.Scan() {
		line := scanner.Text()

		// Line is a mask
		if maskRegex.MatchString(line) {
			// Start with blank mask
			mask = maskRegex.FindStringSubmatch(line)[1]
			continue
		}

		matches := regexp.MustCompile("mem\\[([0-9]+)\\] = ([0-9]+)").FindStringSubmatch(line)

		value, _ := strconv.Atoi(matches[2])

		instruction := instruction{address: matches[1], value: value}

		bits := []rune(fmt.Sprintf("%0*b", maskLength, instruction.value))

		log.Printf("In %v %v", string(bits), instruction.value)
		log.Printf("Mk %v", mask)

		for bitIndex, bitRune := range []rune(mask) {
			bitString := string(bitRune)
			switch bitString {
			case "X":
				{
					continue
				}
			default:
				{
					bits[bitIndex] = bitRune
				}
			}
		}

		maskedInt, err := strconv.ParseUint(string(bits), 2, maskLength)

		if err != nil {
			log.Fatalf("Failed to parse %v", string(bits))
		}

		log.Printf("Ou %v %v", string(bits), maskedInt)

		memory[instruction.address] = int(maskedInt)
	}

	file.Close()

	result := 0

	for _, value := range memory {
		result += value
	}

	log.Printf("Result %v", result)
}
