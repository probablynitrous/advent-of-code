package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	entries := readEntries("input.txt")
	result := entries.getTotal()
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
type Wiring map[int]map[string]bool

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

func (entry Entry) getInputSegmentFrequencies() map[string]string {
	frequencies := make(map[string]int)
	for _, input := range entry.inputs {
		parts := strings.Split(input, "")
		for _, part := range parts {
			frequencies[part] = frequencies[part] + 1
		}
	}

	frequeniesAsStrings := make(map[string]string)

	for letter, frequency := range frequencies {
		frequeniesAsStrings[letter] = strconv.Itoa(frequency)
	}

	return frequeniesAsStrings
}

func (entry Entry) getLetterToSegmentMap() map[string]string {
	// Segment Freq
	// 0 8
	// 1 6
	// 2 8
	// 3 7
	// 4 4
	// 5 9
	// 6 7
	// Determine which letter is which based on frequency
	letterFrequencies := entry.getInputSegmentFrequencies()
	segments := make(map[string]string)
	for letter, frequency := range letterFrequencies {
		switch frequency {
		case "8":
			{
				// Either segment 0 or segment 2
				// Input length 4 will contain segment 2
				four := entry.getFirstInputSignalOfLength(4)
				if strings.Contains(four, letter) {
					segments[letter] = "2"
				} else {
					segments[letter] = "0"
				}
				break
			}
		case "7":
			{
				// Either segment 3 or segment 6
				// Input length 4 will contain segment 3
				four := entry.getFirstInputSignalOfLength(4)
				if strings.Contains(four, letter) {
					segments[letter] = "3"
				} else {
					segments[letter] = "6"
				}
				break
			}
		case "6":
			{
				segments[letter] = "1"
				break
			}
		case "4":
			{
				segments[letter] = "4"
				break
			}
		case "9":
			{
				segments[letter] = "5"
				break
			}
		}
	}

	return segments
}

func (entry Entry) getFirstInputSignalOfLength(length int) string {
	for _, input := range entry.inputs {
		if len(input) == length {
			return input
		}
	}
	return "Not found"
}

func (entry Entry) getSignalToNumberMap() map[string]string {
	signalToSegmentMap := make(map[string]string)

	// Now we have letter to segment map
	// We can create a signal to number map

	for _, signal := range entry.inputs {
		signalToSegmentMap[order(signal)] = entry.signalToSegments(signal)
	}
	return signalToSegmentMap
}

func (entry Entry) signalToSegments(signal string) string {
	// Sort the segment into order
	letterToSegmentMap := entry.getLetterToSegmentMap()

	signalParts := strings.Split(signal, "")

	for index, letter := range signalParts {
		signalParts[index] = letterToSegmentMap[letter]
	}

	sort.Strings(signalParts)

	orderedSegment := strings.Join(signalParts, "")
	switch orderedSegment {
	case "012456":
		{
			return "0"
		}
	case "25":
		{
			return "1"
		}
	case "02346":
		{
			return "2"
		}
	case "02356":
		{
			return "3"
		}
	case "1235":
		{
			return "4"
		}
	case "01356":
		{
			return "5"
		}
	case "013456":
		{
			return "6"
		}
	case "025":
		{
			return "7"
		}
	case "0123456":
		{
			return "8"
		}
	case "012356":
		{
			return "9"
		}
	default:
		{
			log.Println("Found nothing for", orderedSegment)
		}
	}
	return ""
}

func (entry Entry) getOutput() string {
	output := ""
	for _, signal := range entry.outputs {
		output = output + entry.signalToNumber(signal)
	}
	return output
}

func (entry Entry) signalToNumber(signal string) string {
	signalToNumberMap := entry.getSignalToNumberMap()
	return signalToNumberMap[order(signal)]
}

func order(word string) string {
	letters := strings.Split(word, "")
	sort.Strings(letters)
	return strings.Join(letters, "")
}

func (entries Entries) getTotal() int {
	total := 0
	for _, entry := range entries {
		output, _ := strconv.Atoi(entry.getOutput())
		total = total + output
	}
	return total
}
