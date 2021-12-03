package main

import (
	"bufio"
	"os"
	"strconv"
)

func main() {
	instructions := getInstructions("input.txt")
	result := getPowerConsumption(instructions)
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
		if loop < 1 || mostAppearances < appearances {
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
		if loop < 1 || appearances < leastAppearances {
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

func getGammaRate(instructions Instructions) int {
	gammaRateBits := []int{}
	for i := 0; i < len(instructions[0]); i++ {
		gammaRateBits = append(gammaRateBits, instructions.getMostCommonInIndex(i))
	}
	return bitsToDecimal(gammaRateBits)
}

func getEpsilonRate(instructions Instructions) int {
	epsilonRateBits := []int{}
	for i := 0; i < len(instructions[0]); i++ {
		epsilonRateBits = append(epsilonRateBits, instructions.getLeastCommonInIndex(i))
	}
	return bitsToDecimal(epsilonRateBits)
}

func getPowerConsumption(instructions Instructions) int {
	return getGammaRate(instructions) * getEpsilonRate(instructions)
}
