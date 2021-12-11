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

var octopuses = make(map[Grid]int)
var flashes int

func main() {
	data := readFile("input.txt")
	flashes := getFlashes(data)
	fmt.Println(flashes)
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

func getFlashes(data []string) int {
	loadOctopuses(data)
	var syncStep int
	for i := 0; syncStep == 0; i++ {
		increaseValues()
		if count(0,octopuses) == 100 {
			syncStep = i + 1
		}

	}
	printGrid()
	return syncStep
}

func loadOctopuses(data []string) {
	for y, line := range data {
		values := strings.Split(line, "")
		for x, value := range values {
			currPos := Grid{x: x, y: y}
			octopuses[currPos], _ = strconv.Atoi(value)
		}
	}
}

func increaseValues() {
	var f []Grid
	for y := 0; y < 10; y++ {
		for x := 0; x < 10; x++ {
			currPos := Grid{x: x, y: y}
			octopuses[currPos] += 1
			if octopuses[currPos] > 9 {
				flashes += 1
				octopuses[currPos] = 0
				f = append(f, currPos)
			}
		}
	}
	for _, pos := range f {
		increaseAdjacent(pos)
	}
}

func increaseAdjacent(currPos Grid) {
	adjacent := make(map[string]Grid)
	adjacent["n"] = Grid{x: currPos.x, y: currPos.y - 1}
	adjacent["s"] = Grid{x: currPos.x, y: currPos.y + 1}
	adjacent["w"] = Grid{x: currPos.x - 1, y: currPos.y}
	adjacent["e"] = Grid{x: currPos.x + 1, y: currPos.y}
	adjacent["nw"] = Grid{x: currPos.x - 1, y: currPos.y - 1}
	adjacent["ne"] = Grid{x: currPos.x + 1, y: currPos.y - 1}
	adjacent["sw"] = Grid{x: currPos.x - 1, y: currPos.y + 1}
	adjacent["se"] = Grid{x: currPos.x + 1, y: currPos.y + 1}


	for _, newPos := range adjacent {
		if newPos.x >= 0 && newPos.x < 10 && newPos.y >= 0 && newPos.y < 10 {
			if octopuses[newPos] != 0 {
				octopuses[newPos] += 1
				if octopuses[newPos] > 9 {
					flashes += 1
					octopuses[newPos] = 0
					increaseAdjacent(newPos)
				}
			}
		}
	}
}

//for testing
func printGrid() {
	var gridLine string
	for y := 0; y < 10; y++ {
		gridLine = ""
		for x := 0; x < 10; x++ {
			value := strconv.Itoa(octopuses[Grid{x: x, y: y}])
			gridLine += value
		}
		fmt.Println(gridLine)
	}
	fmt.Println()
}

func count(search int, list map[Grid]int) int {
	var counter int
	for _, target := range list {
		if target == search {
			counter += 1
		}
	}
	return counter
}
