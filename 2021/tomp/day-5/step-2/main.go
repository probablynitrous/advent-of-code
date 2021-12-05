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
	data, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}

	step2(dataLines)
}

func step2(data []string) {
	ventGrid := buildGrid(data)

	total := calcTotal(ventGrid)

	fmt.Println(total)
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
	var direction string
	var diff float64
	var xFrom float64
	var yFrom float64
	var xTo float64
	var yTo float64
	var constCo int

	if startPos[0] == endPos[0] {
		direction = "y"
		constCo, _ = strconv.Atoi(startPos[0])
		yFrom, _ = strconv.ParseFloat(startPos[1], 64)
		yTo, _ = strconv.ParseFloat(endPos[1], 64)
		diff = math.Abs(yFrom - yTo)
	} else if startPos[1] == endPos[1] {
		direction = "x"
		constCo, _ = strconv.Atoi(startPos[1])
		xFrom, _ = strconv.ParseFloat(startPos[0], 64)
		xTo, _ = strconv.ParseFloat(endPos[0], 64)
		diff = math.Abs(xFrom - xTo)
	} else {
		direction = "d"
		constCo, _ = strconv.Atoi(startPos[1])
		xFrom, _ = strconv.ParseFloat(startPos[0], 64)
		yFrom, _ = strconv.ParseFloat(startPos[1], 64)
		xTo, _ = strconv.ParseFloat(endPos[0], 64)
		yTo, _ = strconv.ParseFloat(endPos[1], 64)

		diff = math.Abs(xFrom - xTo)
	}

	xmultiplier := getMultiplier(xFrom, xTo)
	ymultiplier := getMultiplier(yFrom, yTo)

	switch direction {
	case "x":
		for i := 0; i <= int(diff); i++ {
			var coords = Grid{x: int(xFrom) + (i * xmultiplier), y: constCo}
			ventGrid[coords] += 1
		}
	case "y":
		for i := 0; i <= int(diff); i++ {
			var coords = Grid{x: constCo, y: int(yFrom) + (i * ymultiplier)}
			ventGrid[coords] += 1
		}
	case "d":
		for i := 0; i <= int(diff); i++ {
			var coords = Grid{x: int(xFrom) + (i * xmultiplier), y: int(yFrom) + (i * ymultiplier)}
			ventGrid[coords] += 1
		}
	}

	return ventGrid
}
