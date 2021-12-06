package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Grid struct {
	x int
	y int
}

func main() {
	data := readFile("input.txt")
	ventGrid := buildGrid(data)
	total := calcTotal(ventGrid)
	fmt.Println(total)
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

func buildGrid(data []string) map[Grid]int {
	ventGrid := make(map[Grid]int)

	for _, line := range data {
		splitLine := strings.Split(line, " -> ")
		startPos := strings.Split(splitLine[0], ",")
		endPos := strings.Split(splitLine[1], ",")

		ventGrid = markLine(ventGrid, startPos, endPos)
	}

	return ventGrid
}

func calcTotal(ventGrid map[Grid]int) int {
	var total int
	for _, value := range ventGrid {
		if value > 1 {
			total += 1
		}
	}
	return total
}

func getMultiplier(start float64, end float64) int {
	if start-end > 0 {
		return -1
	} else {
		return 1
	}
}

func markLine(ventGrid map[Grid]int, startPos []string, endPos []string) map[Grid]int {

	if startPos[0] == endPos[0] {
		ventGrid = markVerticalLine(ventGrid, startPos, endPos)
	} else if startPos[1] == endPos[1] {
		ventGrid = markHorizontalLine(ventGrid, startPos, endPos)
	} else {
		ventGrid = markDiagonalLine(ventGrid, startPos, endPos)
	}

	return ventGrid
}

func markHorizontalLine(ventGrid map[Grid]int, startPos []string, endPos []string) map[Grid]int {
	constCo, _ := strconv.Atoi(startPos[1])
	From, _ := strconv.ParseFloat(startPos[0], 64)
	To, _ := strconv.ParseFloat(endPos[0], 64)
	diff := math.Abs(From - To)

	xmultiplier := getMultiplier(From, To)

	for i := 0; i <= int(diff); i++ {
		var coords = Grid{x: int(From) + (i * xmultiplier), y: constCo}
		ventGrid[coords] += 1
	}

	return ventGrid
}

func markVerticalLine(ventGrid map[Grid]int, startPos []string, endPos []string) map[Grid]int {
	constCo, _ := strconv.Atoi(startPos[0])
	From, _ := strconv.ParseFloat(startPos[1], 64)
	To, _ := strconv.ParseFloat(endPos[1], 64)
	diff := math.Abs(From - To)

	ymultiplier := getMultiplier(From, To)

	for i := 0; i <= int(diff); i++ {
		var coords = Grid{x: constCo, y: int(From) + (i * ymultiplier)}
		ventGrid[coords] += 1
	}

	return ventGrid
}

func markDiagonalLine(ventGrid map[Grid]int, startPos []string, endPos []string) map[Grid]int {
	xFrom, _ := strconv.ParseFloat(startPos[0], 64)
	yFrom, _ := strconv.ParseFloat(startPos[1], 64)
	xTo, _ := strconv.ParseFloat(endPos[0], 64)
	yTo, _ := strconv.ParseFloat(endPos[1], 64)

	diff := math.Abs(xFrom - xTo)

	xmultiplier := getMultiplier(xFrom, xTo)
	ymultiplier := getMultiplier(yFrom, yTo)

	for i := 0; i <= int(diff); i++ {
		var coords = Grid{x: int(xFrom) + (i * xmultiplier), y: int(yFrom) + (i * ymultiplier)}
		ventGrid[coords] += 1
	}

	return ventGrid
}
