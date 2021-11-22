package main

import (
	"bufio"
	"errors"
	"log"
	"os"
	"strconv"
)

const threads = 8

type device struct {
	rating int
}

type adapter struct {
	rating int
}

func (adapter adapter) inputMin() int {
	return adapter.rating - 3
}

func (adapter adapter) inputMax() int {
	return adapter.rating
}

func (device device) inputMin() int {
	return device.rating
}

func (device device) inputMax() int {
	return device.rating + 3
}

func newAdapter(string string) adapter {
	rating, _ := strconv.Atoi(string)
	return adapter{rating: rating}
}

var total int

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	adapters := []adapter{}
	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		adapter := newAdapter(line)
		adapters = append(adapters, adapter)
	}

	total = len(adapters)

	file.Close()

	resultChan := make(chan map[int]int)
	errChan := make(chan error)

	for i := 0; i < threads; i++ {
		go findChainSum(i, 0, adapters, make(map[int]int), resultChan, errChan)
	}

	done := 0

	for {
		if threads <= done {
			break
		}
		err := <-errChan
		if err == nil {
			break
		}

		done++
	}

	log.Println("Expecting result")
	result := <-resultChan

	// Device is always +3
	result[3] = result[3] + 1

	log.Printf("Result %v", result[1]*result[3])
}

func findChainSum(threadIndex int, joltage int, adapters []adapter, result map[int]int, resultChan chan map[int]int, errChan chan error) {
	// If we have no more adapters then we are finished
	if len(adapters) < 1 {
		log.Println("Finished")
		errChan <- nil
		resultChan <- result
	}

	start := threadIndex
	interval := threads

	// Nested thread
	if threadIndex < 0 {
		start = 0
		interval = 1
	}

	// For each adapter
	for index := start; index < len(adapters); index += interval {
		// If it's a potential adapter
		if (adapters[index].inputMin() <= joltage) && (joltage <= adapters[index].inputMax()) {
			var err error
			for _, potentialNextAdapter := range adapters {
				if potentialNextAdapter.inputMax() < adapters[index].rating {
					err = errors.New("All adapters are smaller")
				}
			}

			if err != nil {
				continue
			}

			delta := adapters[index].rating - joltage
			potentialNextAdapters := make([]adapter, len(adapters))
			copy(potentialNextAdapters, adapters)
			potentialNextAdapters = remove(potentialNextAdapters, index)
			nextResultChan := make(chan map[int]int)
			nextErrChan := make(chan error)

			go findChainSum(-1, adapters[index].rating, potentialNextAdapters, result, nextResultChan, nextErrChan)
			err = <-nextErrChan
			if err != nil {
				continue
			}

			result[delta] = result[delta] + 1
			errChan <- nil
			resultChan <- result
		}
	}

	if 0 < threadIndex {
		log.Printf("Finished thread %v", threadIndex)
	}

	errChan <- errors.New("Found no potential adapters")
}

func remove(s []adapter, i int) []adapter {
	s[len(s)-1], s[i] = s[i], s[len(s)-1]
	return s[:len(s)-1]
}
