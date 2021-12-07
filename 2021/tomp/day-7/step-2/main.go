package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := readFile("input.txt")
	fuelRequired := calculateFuel(data)
	fmt.Println(int(fuelRequired))
}

func readFile(fileName string) []string {
	data, _ := os.Open(fileName)
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}
	return dataLines
}

func calculateFuel(data []string) float64 {
	crabPos := splitLine(data[0])
	convergencePoint := findMean(crabPos)
	fuelUsed := math.Min(getFuel(crabPos, convergencePoint), getFuel(crabPos, convergencePoint + 1))

	return fuelUsed
}

func getFuel(crabPos []float64, convergencePoint float64)float64 {
	var fuelUsed float64
	for _, value := range crabPos {

		diff := math.Abs(convergencePoint - value)

		fuelUsed += (diff*(diff+1))/2
	}
	return fuelUsed
}

func splitLine(data string) []float64 {
	dataArr := strings.Split(data, ",")
	crabPos := make([]float64, len(dataArr))
	for i, value := range dataArr {
		crabPos[i], _ = strconv.ParseFloat(value, 64)
	}

	return crabPos
}

func findMean(crabPos []float64) float64 {
	var total float64
	for _, value := range crabPos {
		total += value
	}

	convergencePoint := total / float64(len(crabPos))
	convergencePoint = math.Floor(convergencePoint)
	return convergencePoint
}
