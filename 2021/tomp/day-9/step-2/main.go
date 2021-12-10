package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Grid struct {
	x int
	y int
}

var globalSpotsCovered []Grid

func main() {
	data := readFile("input.txt")
	totalBasins := getBasins(data)
	fmt.Println(totalBasins)
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

func getBasins(data []string) int {
	tubeLevels := buildGrid(data)
	limits := Grid{x: len(data[0]), y: len(data)}

	var basins []int
	lowestPoints := getLowestPoints(tubeLevels, limits)

	for key := range lowestPoints {
		var spotsCovered []Grid
		spotsCovered = findBasin(tubeLevels, key, limits, spotsCovered)
		basins = append(basins, len(spotsCovered))
	}

	largestBasins := getTop3Basins(basins)
	return largestBasins
}

func getLowestPoints(tubeLevels map[Grid]int, limits Grid) map[Grid]int {
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

	return lowestPoints
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

func findBasin(tubeLevels map[Grid]int, currPos Grid, limits Grid, spotsCovered []Grid) []Grid {
	adjacentValues := findAdjacent(tubeLevels, currPos, limits)
	spotsCovered = append(spotsCovered, currPos)
	for key, value := range adjacentValues {
		if value < 9 {
			if !find(key, spotsCovered) {
				spotsCovered = findBasin(tubeLevels, key, limits, spotsCovered)
			}
		}
	}

	return spotsCovered
}

func find(search Grid, list []Grid) bool {
	for _, target := range list {
		if target == search {
			return true
		}
	}
	return false
}
func getTop3Basins(basins []int) int {
	sort.Ints(basins)
	return basins[len(basins)-1] * basins[len(basins)-2] * basins[len(basins)-3]
}
