package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	data, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}
	step1(dataLines)
}

func step1(data []string) {
	bitValue := make(map[int]float64)
	for _, line := range data {
		numbers := strings.Split(line, "")
		for i := 0; i < len(numbers); i++ {
			value, _ := strconv.ParseFloat(numbers[i],64)
			bitValue[i] += value
		}
	}

	var gammaRate string
	var epsilonRate string
	dataLen := float64(len(data))
	for i := 0; i < len(bitValue); i++ {
		frequency := bitValue[i]/dataLen
		switch math.Round(frequency){
		case 1:	
			gammaRate += "1"
			epsilonRate += "0"
		case 0:
			gammaRate += "0"
			epsilonRate += "1"
		}

	}

	gamma,_ := strconv.ParseInt(gammaRate,2,64)
	epsilon,_ := strconv.ParseInt(epsilonRate,2,64)

	fmt.Println(gamma*epsilon)
}
