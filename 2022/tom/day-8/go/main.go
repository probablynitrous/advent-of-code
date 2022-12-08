package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

type Grid struct{
	x,y int
}

var trees = make(map[Grid]int)

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

	maxX := len(data[0]) - 1
	maxY := len(data) - 1

	for y,line := range data {
		for x,c := range line {
			trees[Grid{y,x}] = int(c - '0')
		}
	}
	visibleTrees := 0
	for i,tree := range trees {
		if i.x == 0 || i.y == 0 || i.x == maxX || i.y ==maxY {
			visibleTrees += 1
			continue
		}
		up,_ := lookUp(tree, i) 
		down,_ := lookDown(tree, i, maxY) 
		left,_ := lookLeft(tree, i, maxX) 
		right,_ := lookRight(tree, i)
		if up||down||left||right {
			visibleTrees += 1
		}
	}

	return visibleTrees
}

func step2(data []string) int {
	maxX := len(data[0]) - 1
	maxY := len(data) - 1
	topScore := 0

	for y,line := range data {
		for x,c := range line {
			trees[Grid{y,x}] = int(c - '0')
		}
	}
	visibleTrees := 0
	for i,tree := range trees {
		if i.x == 0 || i.y == 0 || i.x == maxX || i.y ==maxY {
			visibleTrees += 1
			continue
		}
		_,up := lookUp(tree, i) 
		_,down := lookDown(tree, i, maxY) 
		_,left := lookLeft(tree, i, maxX) 
		_,right := lookRight(tree, i)
		score := (up*down*left*right)
		if score > topScore {
			topScore = score
		}
	}

	return topScore
}

func lookUp(curr int, pos Grid) (bool,int) {
	score := 0
	for i := pos.y - 1; i >= 0; i-- {
		score += 1
		if trees[Grid{pos.x,i}] >= curr {
			return false,score
		}
	}
	return true,score
}
func lookDown(curr int, pos Grid, max int) (bool,int) {
	score := 0
	for i := pos.y + 1; i <= max; i++ {
		score += 1
		if trees[Grid{pos.x,i}] >= curr {
			return false,score
		}
	}
	return true,score
}
func lookLeft(curr int, pos Grid, max int) (bool,int) {
	score := 0
	for i := pos.x + 1; i <= max; i++ {
		score += 1
		if trees[Grid{i,pos.y}] >= curr {
			return false,score
		}
	}
	return true,score
}
func lookRight(curr int, pos Grid) (bool,int) {
	score := 0
	for i := pos.x - 1; i >= 0; i-- {
		score += 1
		if trees[Grid{i,pos.y}] >= curr {
			return false,score
		}
	}
	return true,score
}