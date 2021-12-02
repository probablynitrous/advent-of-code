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

	step2(dataLines)
}

func step2(data []string){
	datamap := readArr(data)

	fmt.Println(int(datamap["x"] * datamap["y"]))
}

func readArr(data []string) map[string]float64{
	dataMap := make(map[string]float64)
	
	for _,line := range data {
		rDirection := regexp.MustCompile(`[a-z]`)
		direction := rDirection.FindAllString(line,1)
		
		rValue := regexp.MustCompile(`[0-9]+`)
		value := rValue.FindAllString(line,1)
		valueInt,_ := strconv.ParseFloat(value[0],64)
		switch direction[0] {
		case "u":
			dataMap["a"] -= valueInt
		case "d":
			dataMap["a"] += valueInt
		case "f":
			dataMap["x"] += valueInt
			dataMap["y"] += dataMap["a"] * valueInt
		}
	}
	return dataMap
}



