package main

import (
	"bufio"
	"fmt"
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
	totalRiskLevels := getTotalRiskLevels(data)
	fmt.Println(totalRiskLevels)
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

func getTotalRiskLevels(data []string) int {
	tubeLevels := buildGrid(data)
	limits := Grid{x: len(data[0]), y: len(data)}
	lowestPoints := make(map[Grid]int)

	for y := 0; y < limits.y; y++ {
		for x := 0; x < limits.x; x++ {
			currPos := Grid{x: x, y: y}
			adjacentValues := findAdjacent(tubeLevels, currPos, limits)
			lowestPoint := true
			for _, value := range adjacentValues {
				if value <= tubeLevels[currPos] {
					lowestPoint = false
				}
			}
			if lowestPoint {
				lowestPoints[currPos] = tubeLevels[currPos]
			}
		}
	}
	totalRiskLevel := getTotalRiskLevel(lowestPoints)
	return totalRiskLevel
}

func getTotalRiskLevel(lowestPoints map[Grid]int) int {
	var totalRiskLevel int
	for _, value := range lowestPoints {
		riskLevel := value + 1
		totalRiskLevel += riskLevel
	}

	return totalRiskLevel
}

func buildGrid(data []string) map[Grid]int {
	tubeLevels := make(map[Grid]int)
	for y, line := range data {
		splitLine := strings.Split(line, "")
		for x, level := range splitLine {
			pos := Grid{x: x, y: y}
			tubeLevels[pos], _ = strconv.Atoi(level)
		}
	}

	return tubeLevels
}

func findAdjacent(tubeLevels map[Grid]int, currPos Grid, limits Grid) map[Grid]int {
	adjacentValues := make(map[Grid]int)
	y := currPos.y
	x := currPos.x
	switch y {
	case 0:
		above := Grid{x: x, y: y + 1}
		adjacentValues[above] = tubeLevels[above]
		below := Grid{x: x, y: y - 1}
		adjacentValues[below] = 9
	case limits.y - 1:
		above := Grid{x: x, y: y + 1}
		adjacentValues[above] = 9
		below := Grid{x: x, y: y - 1}
		adjacentValues[below] = tubeLevels[below]
	default:
		above := Grid{x: x, y: y + 1}
		adjacentValues[above] = tubeLevels[above]
		below := Grid{x: x, y: y - 1}
		adjacentValues[below] = tubeLevels[below]
	}

	switch x {
	case 0:
		left := Grid{x: x - 1, y: y}
		adjacentValues[left] = 9
		right := Grid{x: x + 1, y: y}
		adjacentValues[right] = tubeLevels[right]
	case limits.x - 1:
		left := Grid{x: x - 1, y: y}
		adjacentValues[left] = tubeLevels[left]
		right := Grid{x: x + 1, y: y}
		adjacentValues[right] = 9
	default:
		left := Grid{x: x - 1, y: y}
		adjacentValues[left] = tubeLevels[left]
		right := Grid{x: x + 1, y: y}
		adjacentValues[right] = tubeLevels[right]
	}

	return adjacentValues
}
