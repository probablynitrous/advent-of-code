package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	data := readFile("input.txt")
	fuelRequired := calculateFuel(data)
	fmt.Println(fuelRequired)
}

func readFile(fileName string)[]string {
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
	convergencePoint := findMedian(crabPos)
	var fuelUsed float64
	for _,value := range crabPos {
		fuelUsed += math.Abs(convergencePoint - value)
	}

	return fuelUsed
}

func splitLine(data string) []float64 {
	dataArr := strings.Split(data, ",")
	crabPos := make([]float64, len(dataArr))
	for i,value := range dataArr {
		crabPos[i],_ = strconv.ParseFloat(value,64)
	}

	return crabPos
}

func findMedian(crabPos []float64)float64 {
	sort.Float64s(crabPos)
	midPoint := len(crabPos) / 2
	convergencePoint := crabPos[midPoint]
	return convergencePoint
}
