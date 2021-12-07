package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func main() {
	shoal := getShoal("input.txt")
	shoal = shoal.ticks(256)
	result := shoal.getSize()
	println("Result", result)
}

type Fish struct {
	age int
}

type Fishes []Fish

type Shoal map[int]int

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

func createFish(age int) Fish {
	fish := Fish{age: age}
	return fish
}

func getShoal(inputFileName string) Shoal {
	fishes := getFishes(inputFileName)
	shoal := Shoal{}
	for _, fish := range fishes {
		shoal[fish.age] = shoal[fish.age] + 1
	}
	return shoal
}

func (shoal Shoal) tick() Shoal {
	newFish := shoal[0]
	// Move all the fish down
	for age := 0; age < 9; age++ {
		shoal[age] = shoal[age+1]
	}
	// Add the new fish
	shoal[6] = shoal[6] + newFish
	shoal[8] = newFish
	return shoal
}

func (shoal Shoal) getSize() int {
	size := 0
	for _, count := range shoal {
		size = size + count
	}
	return size
}

func (shoal Shoal) ticks(count int) Shoal {
	for i := 0; i < count; i++ {
		shoal = shoal.tick()
	}
	return shoal
}
