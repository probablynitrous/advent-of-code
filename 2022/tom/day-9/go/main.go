package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type Grid struct{
	x,y int
}

var grid = make(map[Grid]int)

func main() {
	data := getInput()
	step1 := step1(data)
	fmt.Println("Step 1: ",step1)
	step2 := step2(data)
	fmt.Println("Step 2: ",step2)
}

func getInput() []string {
	testArg := flag.Bool("test", false, "")
	flag.Parse()
	var test bool = bool(*testArg)

	if test {
		return readFile("../test.txt")
	} else {
		return readFile("../input.txt")
	}
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

func step1(data []string) int {
	tailLocations := make(map[Grid]int)
	currHeadLocation := Grid{0,0}
	currTailLocation := Grid{0,0}
	tailLocations[currTailLocation] = 1;
	for _,line := range data {
		r := regexp.MustCompile(`([A-Z]) ([0-9]+)`)
		m := r.FindAllStringSubmatch(line, -1)[0]
		direction := m[1]
		distance,_ := strconv.Atoi(m[2])

		switch direction {
		case "R":
			for i := 0; i < distance; i++ {
				currHeadLocation = Grid{currHeadLocation.x + 1, currHeadLocation.y}
				if !isAdjacent(currHeadLocation,currTailLocation) {
					currTailLocation = Grid{currHeadLocation.x - 1 ,currHeadLocation.y}
					tailLocations[currTailLocation] += 1
				}
			}
		case "L":
			for i := 0; i < distance; i++ {
				currHeadLocation = Grid{currHeadLocation.x - 1, currHeadLocation.y}
				if !isAdjacent(currHeadLocation,currTailLocation) {
					currTailLocation = Grid{currHeadLocation.x + 1 ,currHeadLocation.y}
					tailLocations[currTailLocation] += 1
				}
			}
		case "U":
			for i := 0; i < distance; i++ {
				currHeadLocation = Grid{currHeadLocation.x , currHeadLocation.y + 1}
				if !isAdjacent(currHeadLocation,currTailLocation) {
					currTailLocation = Grid{currHeadLocation.x ,currHeadLocation.y - 1}
					tailLocations[currTailLocation] += 1
				}
			}
		case "D":
			for i := 0; i < distance; i++ {
				currHeadLocation = Grid{currHeadLocation.x , currHeadLocation.y - 1}
				if !isAdjacent(currHeadLocation,currTailLocation) {
					currTailLocation = Grid{currHeadLocation.x ,currHeadLocation.y + 1}
					tailLocations[currTailLocation] += 1
				}
			}
		}
	}
	return len(tailLocations)
}

func step2(data []string) int {
	tailLocations := make(map[Grid]int)
	var currTailLocation [10]Grid

	for i := 0; i < 10; i++ {
		currTailLocation[i] = Grid{0,0}
	}

	tailLocations[currTailLocation[9]] = 1;
	for _,line := range data {
		r := regexp.MustCompile(`([A-Z]) ([0-9]+)`)
		m := r.FindAllStringSubmatch(line, -1)[0]
		direction := m[1]
		distance,_ := strconv.Atoi(m[2])

		switch direction {
		case "R":
			for i := 0; i < distance; i++ {
				currTailLocation[0] = Grid{currTailLocation[0].x + 1, currTailLocation[0].y}
				for j := 1; j <= 9; j++ {
					if !isAdjacent(currTailLocation[j-1],currTailLocation[j]) {
						currTailLocation[j] = getNewGrid(currTailLocation[j-1],currTailLocation[j])
						if j == 9 {
							//fmt.Println(currTailLocation[j])
							tailLocations[currTailLocation[j]] += 1
						}
					}
				}
			}
		case "L":
			for i := 0; i < distance; i++ {
				currTailLocation[0] = Grid{currTailLocation[0].x - 1, currTailLocation[0].y}
				for j := 1; j <= 9; j++ {
					if !isAdjacent(currTailLocation[j-1],currTailLocation[j]) {
						currTailLocation[j] = getNewGrid(currTailLocation[j-1],currTailLocation[j])
						if j == 9 {
							//fmt.Println(currTailLocation[j])
							tailLocations[currTailLocation[j]] += 1
						}
					}
				}
			}
		case "U":
			for i := 0; i < distance; i++ {
				currTailLocation[0] = Grid{currTailLocation[0].x , currTailLocation[0].y + 1}
				for j := 1; j <= 9; j++ {
					if !isAdjacent(currTailLocation[j-1],currTailLocation[j]) {
						currTailLocation[j] = getNewGrid(currTailLocation[j-1],currTailLocation[j])
						if j == 9 {
							tailLocations[currTailLocation[j]] += 1
						}
					}
				}
			}
		case "D":
			for i := 0; i < distance; i++ {
				currTailLocation[0] = Grid{currTailLocation[0].x , currTailLocation[0].y - 1}
				for j := 1; j <= 9; j++ {
					if !isAdjacent(currTailLocation[j-1],currTailLocation[j]) {
						currTailLocation[j] = getNewGrid(currTailLocation[j-1],currTailLocation[j])
						if j == 9 {
							tailLocations[currTailLocation[j]] += 1
						}
					}
				}
			}
		}
	}
	return len(tailLocations)
}

func isAdjacent(currHeadLocation Grid, currTailLocation Grid) bool {
	if ((Abs(currTailLocation.x - currHeadLocation.x) <= 1) && 
		 (Abs(currTailLocation.y - currHeadLocation.y) <= 1)) {
			return true
		 }
	return false
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func getNewGrid(prevPos Grid, currPos Grid) Grid{
	newX := findIfPos(prevPos.x - currPos.x)
	newY := findIfPos(prevPos.y - currPos.y)
	return Grid{currPos.x + newX, currPos.y + newY}
}

func findIfPos(value int) int {
	if value == 0 {
		return 0
	}
	if value > 0 {
		return 1
	}
	return -1
}