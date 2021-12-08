package main

import (
	"bufio"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

// 95476248
// Too high
func main() {
	crabs := getCrabs("input.txt")
	result := crabs.getTotalFuelToMean()
	println("Result", result)
}

type Crab struct {
	x int
}

type Crabs []Crab

func getCrabs(inputFileName string) Crabs {
	// Assume we can always open the file
	file, _ := os.Open(inputFileName)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	crabs := Crabs{}

	// Parse lines
	scanner.Scan()
	line := scanner.Text()

	for _, crabStringValue := range strings.Split(line, ",") {
		crabIntValue, _ := strconv.Atoi(crabStringValue)
		crabs = append(crabs, createCrab(crabIntValue))
	}

	sort.Slice(crabs, func(i, j int) bool {
		return crabs[i].x < crabs[j].x
	})

	return crabs
}

func createCrab(fuel int) Crab {
	crab := Crab{fuel}
	return crab
}

func (crabs Crabs) getMeanX() int {
	fuelSum := 0
	for _, crab := range crabs {
		fuelSum = fuelSum + crab.x
	}
	mean := math.Floor(float64(fuelSum) / float64(len(crabs)))
	// No idea why floor is correct and round is wrong
	// It is though
	return int(mean)
}

func (crabs Crabs) getTotalFuelToX(x int) int {
	fuelTotal := 0
	for _, crab := range crabs {
		// Converting to float just to do math.Abs is pure crabs
		delta := x - crab.x
		if delta < 0 {
			delta = 0 - delta
		}

		for cost := 1; cost <= delta; cost++ {
			fuelTotal = fuelTotal + cost
		}
	}
	return fuelTotal
}

func (crabs Crabs) getTotalFuelToMean() int {
	return crabs.getTotalFuelToX(crabs.getMeanX())
}
