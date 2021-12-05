package main

import (
	"fmt"
	"os"
	"bufio"
	"strconv"
)

func main(){
	data := readFile("input.txt")
	step1(data)
	step2(data)
}

func readFile(fileName string)[]int{
	data, _ := os.Open(fileName)
	scanner := bufio.NewScanner(data)

	var dataLines []int
	for scanner.Scan() {
		line,_ := strconv.Atoi(scanner.Text())
		dataLines = append(dataLines, line)
	}
	return dataLines
}

func step1(data []int)  {
	previous := 0
	noIncrease := 0

	for _,line := range data {
		if line > previous && previous != 0 {
			noIncrease++
		}
		previous = line
	}

	fmt.Println(noIncrease)
}

func step2(data []int)  {
	previousWindow := 0
	noIncrease := 0
	currWindow := 0

	for i := 1; i + 1 < len(data); i++ {
		currWindow = data[i - 1] + data[i] + data[i + 1]
		if currWindow > previousWindow && previousWindow != 0 {
			noIncrease++
		}
		previousWindow = currWindow
	}
	fmt.Println(noIncrease)
}