package main

import (
	"bufio"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

// 474197
// Too high
func main() {
	crabs := getCrabs("input.txt")
	result := crabs.getTotalFuelToMedian()
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
	fuelSum = fuelSum / len(crabs)
	return fuelSum
}

func (crabs Crabs) getMedianX() int {
	index := len(crabs) / 2
	return crabs[index].x
}

func (crabs Crabs) getTotalFuelToMedian() int {
	median := crabs.getMedianX()
	fuelTotal := 0
	for _, crab := range crabs {
		fuelTotal = fuelTotal + int(math.Abs(float64(median-crab.x)))
	}
	return fuelTotal
}
