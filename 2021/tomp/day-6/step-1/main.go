package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := readFile("test.txt")
	fishCount := trackFish(data)
	fmt.Println(fishCount)
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

func trackFish(data []string) int {
	fishArr := splitLine(data[0])
	fishArr = breed80Days(fishArr)
	return len(fishArr)
}

func splitLine(line string) []int {
	lineTxt := strings.Split(line, ",")
	var fishArr []int
	for _, fish := range lineTxt {
		fishInt, _ := strconv.Atoi(fish)
		fishArr = append(fishArr, fishInt)
	}
	return fishArr
}

func breed80Days(fishArr []int) []int {
	for i := 0; i < 80; i++ {
		fishArr = breed(fishArr)
	}
	return fishArr
}

func breed(fishArr []int) []int {
	newFishArr := fishArr
	for i, fish := range fishArr {
		if fish == 0 {
			newFishArr[i] = 6
			newFishArr = append(newFishArr, 8)
		} else {
			newFishArr[i] -= 1
		}
	}
	return newFishArr
}
