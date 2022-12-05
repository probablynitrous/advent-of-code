package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)


func main() {
	data := readFile("input.txt")
	step1 := step1(data)
	fmt.Println(step1)
	step2 := step2(data)
	fmt.Println(step2)
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

func step1(data []string) string {
	
	stacks,breakLine := getInitialStack(data)

	r := regexp.MustCompile(`move ([0-9]+) from ([0-9]+) to ([0-9]+)`)

	for _,line := range data[breakLine+1:] {
		movements := r.FindAllStringSubmatch(line,-1)[0]

		moveFrom,_ := strconv.Atoi(movements[2])
		moveTo,_ := strconv.Atoi(movements[3])
		moveAmount,_ := strconv.Atoi(movements[1])

		var movedList []string
		if moveAmount == len(stacks[moveFrom-1]) {
			movedList = stacks[moveFrom-1]
		} else {
			movedList = stacks[moveFrom-1][:moveAmount]
		}

		for _,moved := range movedList {
			stacks[moveTo -1] = append([]string{moved},stacks[moveTo-1]... )
			stacks[moveFrom-1] = stacks[moveFrom-1][1:]
		}	

	}
	var topStack string

	for _,stack := range stacks {
		if len(stack) > 0 {
			topStack += stack[0]
		}
	}
	return topStack
}

func step2(data []string) string {
	
	stacks,breakLine := getInitialStack(data)

	r := regexp.MustCompile(`move ([0-9]+) from ([0-9]+) to ([0-9]+)`)

	for _,line := range data[breakLine+1:] {
		movements := r.FindAllStringSubmatch(line,-1)[0]

		moveFrom,_ := strconv.Atoi(movements[2])
		moveTo,_ := strconv.Atoi(movements[3])
		moveAmount,_ := strconv.Atoi(movements[1])

		var movedList []string
		if moveAmount == len(stacks[moveFrom-1]) {
			movedList = stacks[moveFrom-1]
		} else {
			movedList = stacks[moveFrom-1][:moveAmount]
		}
		stacks[moveFrom-1] = stacks[moveFrom-1][len(movedList):]

		stacks[moveTo-1] = append(copyArr(movedList),stacks[moveTo-1]...)
		
	}
	var topStack string

	for _,stack := range stacks {
		if len(stack) > 0 {
			topStack += stack[0]
		}
	}
	return topStack
}

func getInitialStack(data []string) ([][]string,int) {
	var initialStackLines []string
	var breakLine int
	for i,line := range data {
		if line == "" {
			breakLine = i
			break
		}
		initialStackLines = append(initialStackLines, line)
	}
	var stackCount int = len(initialStackLines)
	var stacks = make([][]string,stackCount)
	

	for _,stackRow := range initialStackLines[:stackCount-1] {
		for i,c := range stackRow {
			letter := string(c)
			valid,_ := regexp.MatchString(`[A-Z]`, letter)
			if !valid {
				continue
			}
			stack := ((i + 3) / 4) - 1
			stacks[stack] = append(stacks[stack], letter)
		}
	}
	
	return stacks,breakLine
}


func copyArr(c []string) []string {
	s := make([]string, len(c))
	copy(s, c)
	return s
}