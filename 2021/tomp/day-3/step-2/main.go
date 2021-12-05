package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Ratings struct {
	oRating  int
	coRating int
}

func main() {
	data := readFile("input.txt")
	ratings := getRatings(data)
	fmt.Println(ratings.oRating * ratings.coRating)
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

func getRatings(data []string) Ratings {
	oRating := getRating(data, 1)
	coRating := getRating(data, 0)

	oRatingInt, _ := strconv.ParseInt(oRating, 2, 64)
	coRatingInt, _ := strconv.ParseInt(coRating, 2, 64)

	return Ratings{oRating: int(oRatingInt), coRating: int(coRatingInt)}
}

func getBitValue(data []string)map[int]float64{
	bitValue := make(map[int]float64)
	for _, line := range data {
		numbers := strings.Split(line, "")
		for i := 0; i < len(numbers); i++ {
			value, _ := strconv.ParseFloat(numbers[i], 64)
			bitValue[i] += value
		}
	}
	return bitValue
}

func getMostUsed(data []string, defaultValue float64) map[int]float64 {
	bitValue := getBitValue(data)
	
	mostUsed := make(map[int]float64)
	dataLen := float64(len(data))
	for i := 0; i < len(bitValue); i++ {
		frequency := bitValue[i] / dataLen
		switch math.Round(frequency) {
		case 1:
			mostUsed[i] = defaultValue
		case 0:
			mostUsed[i] = 1 - defaultValue
		}
	}

	return mostUsed
}

func getRating(data []string, defaultValue float64) string {
	sample := data
	for i := 0; len(sample) > 1; i++ {
		var newSample []string
		mostUsed := getMostUsed(sample, defaultValue)[i]

		for _, line := range sample {
			linePos := strings.Split(line, "")
			if linePos[i] == fmt.Sprint(mostUsed) {
				newSample = append(newSample, line)
			}
		}
		sample = newSample
	}
	return sample[0]
}
