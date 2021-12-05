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

	step1(dataLines)
}

func step1(data []string) {
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

		if startPos[0] != endPos[0] && startPos[1] != endPos[1] {
			continue
		}

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

func markLine(ventGrid map[Grid]int, startPos []string, endPos []string) map[Grid]int {

	var direction string
	var diff float64
	var startInt float64
	var endInt float64
	var constCo int
	if startPos[0] == endPos[0] {
		direction = "x"
		constCo, _ = strconv.Atoi(startPos[0])
		startInt, _ = strconv.ParseFloat(startPos[1], 64)
		endInt, _ = strconv.ParseFloat(endPos[1], 64)

		diff = math.Abs(startInt - endInt)
	} else {
		direction = "y"
		constCo, _ = strconv.Atoi(startPos[1])
		startInt, _ = strconv.ParseFloat(startPos[0], 64)
		endInt, _ = strconv.ParseFloat(endPos[0], 64)

		diff = math.Abs(startInt - endInt)
	}

	var multiplier int
	if startInt-endInt > 0 {
		multiplier = -1
	} else {
		multiplier = 1
	}

	switch direction {
	case "y":
		for i := 0; i <= int(diff); i++ {
			var coords = Grid{x: int(startInt) + (i * multiplier), y: constCo}
			ventGrid[coords] += 1
		}
	case "x":
		for i := 0; i <= int(diff); i++ {
			var coords = Grid{x: constCo, y: int(startInt) + (i * multiplier)}
			ventGrid[coords] += 1
		}
	}

	return ventGrid
}
