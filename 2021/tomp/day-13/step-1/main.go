package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Grid struct {
	x int
	y int
}

type Limits struct {
	x int
	y int
}

type Fold struct {
	dimension string
	place     int
}

var dimensions Limits

var foldOrder []Fold

func main() {
	data := readFile("input.txt")
	dots := countDots(data)
	fmt.Println(dots)
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

func setInitialValues(data []string) []Grid {
	var initialGrid []Grid
	for _, line := range data {
		if line != "" {
			if !strings.Contains(line, "fold") {
				corrodinates := strings.Split(line, ",")
				x, _ := strconv.Atoi(corrodinates[0])
				y, _ := strconv.Atoi(corrodinates[1])
				initialGrid = append(initialGrid, Grid{x: x, y: y})
			} else {
				regex, _ := regexp.Compile("[0-9]+")
				place, _ := strconv.Atoi(regex.FindString(line))
				if strings.Contains(line, "x") {
					foldOrder = append(foldOrder, Fold{dimension: "x", place: place})
				} else {
					foldOrder = append(foldOrder, Fold{dimension: "y", place: place})
				}
			}
		}
	}

	return initialGrid
}

func countDots(data []string) int {
	initialGrid := setInitialValues(data)
	dimensions = findDimensions()
	folded := initialGrid

	switch foldOrder[0].dimension {
	case "x":
		folded = foldX(folded, foldOrder[0].place)
	case "y":
		folded = foldY(folded, foldOrder[0].place)

	}

	return len(folded)
}

func foldX(input []Grid, foldPoint int) []Grid {
	var output []Grid
	for _, currPos := range input {
		if currPos.x > foldPoint {
			newX := dimensions.x  - currPos.x
			newPos := Grid{x: newX, y: currPos.y}
			if !find(newPos, output) {
				output = append(output, newPos)
			}
		} else {
			if !find(currPos, output) {
				output = append(output, currPos)
			}
		}
	}
	dimensions = Limits{x: foldPoint - 1, y: dimensions.y}
	return output
}

func foldY(input []Grid, foldPoint int) []Grid {
	var output []Grid
	for _, currPos := range input {
		if currPos.y > foldPoint {
			newY := dimensions.y - currPos.y
			newPos := Grid{x: currPos.x, y: newY}
			if !find(newPos, output) {
				output = append(output, newPos)
			}
		} else {
			if !find(currPos, output) {
				output = append(output, currPos)
			}
		}
	}
	dimensions = Limits{x: dimensions.x, y: foldPoint - 1}

	return output
}

func findDimensions() Limits {
	x := foldOrder[findFirst("x",foldOrder)].place * 2
	y := foldOrder[findFirst("y",foldOrder)].place * 2

	return Limits{x: x, y: y}
}

func findFirst(search string, list []Fold) int {
	for i, target := range list {
		if target.dimension == search {
			return i
		}
	}
	return len(list)
}


func find(search Grid, list []Grid) bool {
	for _, target := range list {
		if target == search {
			return true
		}
	}
	return false
}

func findValue(search Grid, list []Grid) int {
	for i, target := range list {
		if target == search {
			return i
		}
	}
	return len(list)
}