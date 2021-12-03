package main

import (
	"bufio"
	"os"
	"strconv"
)

func main() {
	instructions := getInstructions("input.txt")
	result := getLifeSupportRating(instructions)
	println("Result", result)
}

type Instruction []int
type Instructions []Instruction

func getInstructions(inputFileName string) Instructions {
	// Assume we can always open the file
	file, _ := os.Open(inputFileName)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	instructions := Instructions{}

	// Parse lines
	for scanner.Scan() {
		line := scanner.Text()
		runes := []rune(line)
		ints := runesToInts(runes)
		instructions = append(instructions, Instruction(ints))
	}

	return instructions
}

func runesToInts(runes []rune) Instruction {
	ints := Instruction{}
	for _, value := range runes {
		runeInt, _ := strconv.Atoi(string(value))
		ints = append(ints, runeInt)
	}
	return ints
}

func (instructions Instructions) getMostCommonInIndex(index int) int {
	appearances := make(map[int]int)
	for _, value := range instructions {
		appearances[value[index]] = appearances[value[index]] + 1
	}
	mostCommon := 0
	mostAppearances := 0
	loop := 0
	for key, appearances := range appearances {
		if (loop < 1) || (mostAppearances < appearances) || (mostAppearances == appearances && mostCommon < key) {
			mostCommon = key
			mostAppearances = appearances
		}
		loop = loop + 1
	}
	return mostCommon
}

func (instructions Instructions) getLeastCommonInIndex(index int) int {
	appearances := make(map[int]int)
	for _, value := range instructions {
		appearances[value[index]] = appearances[value[index]] + 1
	}
	leastCommon := 0
	leastAppearances := 0
	loop := 0
	for key, appearances := range appearances {
		if (loop < 1) || (appearances < leastAppearances) || (leastAppearances == appearances && key < leastCommon) {
			leastCommon = key
			leastAppearances = appearances
		}
		loop = loop + 1
	}
	return leastCommon
}

func bitsToDecimal(bits []int) int {
	bitValue := 1
	decimalValue := 0

	for i := len(bits) - 1; -1 < i; i-- {
		if bits[i] == 1 {
			decimalValue = decimalValue + bitValue
		}
		bitValue = bitValue * 2
	}
	return decimalValue
}

func getOxygenGeneratorRating(instructions Instructions) int {
top:
	for bitIndex := 0; bitIndex < len(instructions[0]); bitIndex++ {
		matchedInstructions := Instructions{}
		mostCommonBit := instructions.getMostCommonInIndex(bitIndex)
		for _, instruction := range instructions {
			if instruction[bitIndex] == mostCommonBit {
				matchedInstructions = append(matchedInstructions, instruction)
			}
		}
		instructions = matchedInstructions
		if len(instructions) < 2 {
			break top
		}
	}
	return bitsToDecimal(instructions[0])
}

func getCO2ScrubberRating(instructions Instructions) int {
top:
	for bitIndex := 0; bitIndex < len(instructions[0]); bitIndex++ {
		matchedInstructions := Instructions{}
		leastCommonBit := instructions.getLeastCommonInIndex(bitIndex)
		for _, instruction := range instructions {
			if instruction[bitIndex] == leastCommonBit {
				matchedInstructions = append(matchedInstructions, instruction)
			}
		}
		instructions = matchedInstructions
		if len(instructions) < 2 {
			break top
		}
	}
	return bitsToDecimal(instructions[0])
}

func getLifeSupportRating(instructions Instructions) int {
	return getCO2ScrubberRating(instructions) * getOxygenGeneratorRating(instructions)
}
