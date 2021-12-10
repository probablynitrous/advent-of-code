package main

import (
	"bufio"
	"os"
	"strings"
)

func main() {
	entries := readEntries("input.txt")
	result := entries.countOutputSignalsOfLengths([]int{2, 3, 4, 7})
	println("Result", result)
}

func readEntries(inputFileName string) Entries {
	// Assume we can always open the file
	file, _ := os.Open(inputFileName)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	entries := Entries{}

	// Parse lines
	for scanner.Scan() {
		line := scanner.Text()
		entries = append(entries, lineToEntry(line))
	}

	return entries
}

type Entry struct {
	inputs  []string
	outputs []string
}

type Entries []Entry

func lineToEntry(line string) Entry {
	parts := strings.Split(line, " | ")
	signals := strings.Split(parts[0], " ")
	outputs := strings.Split(parts[1], " ")
	entry := Entry{signals, outputs}
	return entry
}

func (entry Entry) countOutputSignalsOfLength(length int) int {
	count := 0
	for _, output := range entry.outputs {
		if len(output) == length {
			count = count + 1
		}
	}
	return count
}

func (entry Entry) countOutputSignalsOfLengths(lengths []int) int {
	count := 0
	for _, length := range lengths {
		count = count + entry.countOutputSignalsOfLength(length)
	}
	return count
}

func (entries Entries) countOutputSignalsOfLengths(lengths []int) int {
	count := 0
	for _, entry := range entries {
		count = count + entry.countOutputSignalsOfLengths(lengths)
	}
	return count
}
