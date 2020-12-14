package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	var departures []int

	var arrival int

	index := 0
	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		if index == 0 {
			arrival, _ = strconv.Atoi(line)
		} else {
			for _, s := range strings.Split(line, ",") {
				if s != "x" {
					departure, _ := strconv.Atoi(s)
					departures = append(departures, departure)
				}
			}
		}
		index++
	}

	result := 0

	time := arrival

out:
	for {
		for _, departure := range departures {
			if time%departure == 0 {
				result = departure * (time - arrival)
				break out
			}
		}
		time++
	}

	log.Printf("Result %v", result)

	file.Close()
}
