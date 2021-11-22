package main

import (
	"bufio"
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

	acc := exec(0, 0, &instructions)

	println("Result", acc)
}

func exec(index int, acc int, instructions *[]instruction) int {
	if len(*instructions) <= index {
		log.Fatal(fmt.Sprintf("Index %v out of bounds", index))
	}

	if (*instructions)[index].executed == true {
		log.Println("Instruction has already been executed")
		return acc
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
	instruction := instruction{executed: false}
	matches := regex.FindStringSubmatch(line)
	instruction.operation = matches[1]
	value, _ := strconv.Atoi(matches[2])
	instruction.value = value
	return instruction
}
