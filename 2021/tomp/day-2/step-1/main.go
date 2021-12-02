package main

import (
	"fmt"
	"os"
	"bufio"
	"regexp"
	"strconv"
)

func main(){
	data, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}

	step1(dataLines)
	//step2(dataLines)
}

func step1(data []string){
	datamap := readArr(data)
	yPos := datamap["down"] - datamap["up"]
	xPos := datamap["forward"]

	fmt.Println(int(xPos * yPos))
}

func readArr(data []string) map[string]float64{
	dataMap := make(map[string]float64)
	
	for _,line := range data {
		rDirection := regexp.MustCompile(`[a-z]+`)
		direction := rDirection.FindAllString(line,1)
		
		rValue := regexp.MustCompile(`[0-9]+`)
		value := rValue.FindAllString(line,1)
		valueInt,_ := strconv.ParseFloat(value[0],64)
		dataMap[direction[0]] += valueInt
	}
	return dataMap
}



