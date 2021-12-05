package main

import (
	"fmt"
	"os"
	"bufio"
	"regexp"
	"strconv"
)

func main(){
	data := readFile("input.txt")
	datamap := readArr(data)
	yPos := datamap["down"] - datamap["up"]
	xPos := datamap["forward"]

	fmt.Println(int(xPos * yPos))
}

func readFile(fileName string)[]string {
	data, _ := os.Open(fileName)
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}
	return dataLines
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



