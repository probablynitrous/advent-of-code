package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

var regex = regexp.MustCompile("^([a-z]+) (\\+[0-9]+|-[0-9]+)$")

type instruction struct {
	operation string
	value     int
	executed  bool
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	instructions := []instruction{}

	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		instructions = append(instructions, newInstruction(line))
	}

	file.Close()

	acc, err := findCorrupted(instructions)

	if err != nil {
		log.Fatal(err.Error())
	}

	println("Result", acc)
}

func findCorrupted(instructions []instruction) (int, error) {
	// For each instruction
	for index := range instructions {
		// Make a copy of the slice
		modifiedInstructions := make([]instruction, len(instructions))
		copy(modifiedInstructions, instructions)
		switch modifiedInstructions[index].operation {
		case "nop":
			{
				// Replace with jmp and test
				modifiedInstructions[index].operation = "jmp"
				// Find acc or error
				acc, err := exec(0, 0, &modifiedInstructions)
				if err != nil {
					break
				}
				return acc, nil
			}
		case "jmp":
			{
				// Replace with nop and test
				modifiedInstructions[index].operation = "nop"
				// Find acc or error
				acc, err := exec(0, 0, &modifiedInstructions)
				if err != nil {
					break
				}
				return acc, nil
			}
		}
	}

	return 0, errors.New("Failed to find corruption")
}

func exec(index int, acc int, instructions *[]instruction) (int, error) {
	if len(*instructions) < index {
		message := fmt.Sprintf("Index %v out of bounds", index)
		log.Println(message)
		return acc, errors.New(message)
	}

	if index == len(*instructions) {
		log.Println("Program has terminated successfully")
		return acc, nil
	}

	if (*instructions)[index].executed == true {
		message := "Instruction has already been executed"
		log.Println(message)
		return acc, errors.New(message)
	}

	(*instructions)[index].executed = true

	switch (*instructions)[index].operation {
	case "nop":
		{
			index++
			break
		}
	case "acc":
		{
			acc += (*instructions)[index].value
			index++
			break
		}
	case "jmp":
		{
			index += (*instructions)[index].value
			break
		}
	default:
		{
			log.Fatal("Unknown operation when executing instructions")
		}
	}

	return exec(index, acc, instructions)
}

func newInstruction(line string) instruction {
	instruction := instruction{}
	matches := regex.FindStringSubmatch(line)
	instruction.operation = matches[1]
	value, _ := strconv.Atoi(matches[2])
	instruction.value = value
	return instruction
}
