package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	instructions := readInstructions("input.txt")
	submarine := Submarine{}
	for _, instruction := range instructions {
		submarine.move(instruction)
	}
	println("Result", submarine.getProduct())
}

type Submarine struct {
	x   int
	y   int
	aim int
}

type Instruction struct {
	direction string
	value     int
}

func (submarine *Submarine) move(instruction Instruction) {
	switch instruction.direction {
	case "forward":
		{
			submarine.x = submarine.x + instruction.value
			submarine.y = submarine.y + (submarine.aim * instruction.value)
		}
	case "down":
		{
			submarine.aim = submarine.aim - instruction.value

		}
	case "up":
		{
			submarine.aim = submarine.aim + instruction.value
		}
	default:
		{
			log.Fatalf("Unknown direction %v in instruction", instruction.direction)
		}
	}
}

func (submarine *Submarine) getDepth() int {
	return -submarine.y
}

func (submarine *Submarine) getProduct() int {
	return -submarine.y * submarine.x
}

func readInstructions(inputFileName string) []Instruction {
	// Assume we can always open the file
	file, _ := os.Open(inputFileName)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	instructions := []Instruction{}

	// Parse lines
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		direction := parts[0]
		value, _ := strconv.Atoi(parts[1])
		instructions = append(instructions, Instruction{direction, value})
	}

	return instructions
}
