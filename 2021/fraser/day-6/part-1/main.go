package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func main() {
	fishes := getFishes("input.txt")
	fishes = fishes.ticks(80)
	result := len(fishes)
	println("Result", result)
}

type Fish struct {
	age int
}

type Fishes []Fish

func getFishes(inputFileName string) Fishes {
	// Assume we can always open the file
	file, _ := os.Open(inputFileName)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	fishes := Fishes{}

	// Parse lines
	scanner.Scan()
	line := scanner.Text()

	for _, fishStringValue := range strings.Split(line, ",") {
		fishIntValue, _ := strconv.Atoi(fishStringValue)
		fishes = append(fishes, createFish(fishIntValue))
	}
	return fishes
}

func (fish Fish) tick() (Fish, bool) {
	fish.age = fish.age - 1
	if fish.age < 0 {
		fish.age = 6
		return fish, true
	}
	return fish, false
}

func createFish(age int) Fish {
	fish := Fish{age: age}
	return fish
}

func createNewFish() Fish {
	return createFish(8)
}

func (fishes Fishes) tick() Fishes {
	for index, fish := range fishes {
		fish, shouldCreateNewFish := fish.tick()
		fishes[index] = fish
		if shouldCreateNewFish {
			fishes = append(fishes, createNewFish())
		}
	}
	return fishes
}

func (fishes Fishes) ticks(count int) Fishes {
	for i := 0; i < count; i++ {
		fishes = fishes.tick()
	}
	return fishes
}
