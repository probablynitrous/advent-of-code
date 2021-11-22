package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
)

const threads = 8

type adapter struct {
	rating int
}
type adapters []adapter

func (a adapters) Len() int           { return len(a) }
func (a adapters) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a adapters) Less(i, j int) bool { return a[i].rating < a[j].rating }

func (adapter adapter) inputMin() int {
	return adapter.rating - 3
}

func (adapter adapter) inputMax() int {
	return adapter.rating
}

func newAdapter(string string) adapter {
	rating, _ := strconv.Atoi(string)
	return adapter{rating: rating}
}

var children = make(map[int]int)

var result = 0

var target int

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	adapters := adapters{}
	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		adapter := newAdapter(line)
		adapters = append(adapters, adapter)
	}

	file.Close()

	sort.Sort(adapters)

	target = adapters[len(adapters)-1].rating

	result := findPossibleChains(0, adapters)

	log.Printf("Result %v", result)
}

func findPossibleChains(joltage int, adapters []adapter) int {

	if count, ok := children[joltage]; ok {
		return count
	}

	count := 0

	// If we have are at the last adapter then we are finished in this tree
	if joltage == target {
		return 1
	}

	for index, a := range adapters {
		// If it's not a possible adapter
		if joltage < a.inputMin() || a.inputMax() < joltage {
			continue
		}
		remainingAdapters := make([]adapter, len(adapters))
		copy(remainingAdapters, adapters)
		remainingAdapters = remove(remainingAdapters, index)
		childCount := findPossibleChains(a.rating, remainingAdapters)
		children[joltage] += childCount
		count = count + childCount
	}
	return count
}

func remove(s []adapter, i int) []adapter {
	s[len(s)-1], s[i] = s[i], s[len(s)-1]
	return s[:len(s)-1]
}
