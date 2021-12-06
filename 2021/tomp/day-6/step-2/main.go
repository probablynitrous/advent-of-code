package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := readFile("input.txt")
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
	lifeSpan := splitLine(data[0])
	lifeSpan = breed256Days(lifeSpan)
	total := getTotalFish(lifeSpan)
	return total
}

func splitLine(line string) [9]int {
	lineTxt := strings.Split(line, ",")
	var lifeSpan [9]int
	for _, fish := range lineTxt {
		fishInt, _ := strconv.Atoi(fish)
		lifeSpan[fishInt] += 1
	}
	return lifeSpan
}

func breed256Days(lifeSpan [9]int) [9]int {
	for i := 0; i < 256; i++ {
		lifeSpan = breed(lifeSpan)
	}
	return lifeSpan
}

func breed(lifeSpan [9]int) [9]int {
	var newLifeSpan [9]int
	for i, oldValue := range lifeSpan {
		if i == 0 {
			newLifeSpan[8] = oldValue
			newLifeSpan[6] += oldValue
		} else {
			newLifeSpan[i-1] += oldValue
		}
	}

	return newLifeSpan
}

func getTotalFish(lifeSpan [9]int) int {
	var total int
	for _, value := range lifeSpan {
		total += value
	}

	return (total)
}
