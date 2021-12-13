package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

var routes = make(map[string][]string)

func main() {
	data := readFile("input.txt")
	routes := getRoutes(data)
	fmt.Println(routes)
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

func getRoutes(data []string) int {
	for _, line := range data {
		route := strings.Split(line, "-")
		routes[route[0]] = append(routes[route[0]], route[1])
		routes[route[1]] = append(routes[route[1]], route[0])
	}
	var visited []string
	var singleVisited bool = false

	totalRoutes := findCaves(visited, "start", singleVisited)
	return totalRoutes
}

func findCaves(visited []string, currCave string, singleVisited bool) int {
	if currCave == "end" {
		return 1
	}
	if find(currCave, visited) {
		if IsLower(currCave) {
			if !singleVisited {
				singleVisited = true
			} else {
				return 0
			}
		}
		if currCave == "start" {
			return 0
		}
	}
	visited = append(visited, currCave)
	var totalCaves int
	for _, nextCave := range routes[currCave] {
		totalCaves += findCaves(visited, nextCave, singleVisited)
	}
	return totalCaves
}

func find(search string, list []string) bool {
	for _, target := range list {
		if target == search {
			return true
		}
	}
	return false
}

func IsLower(input string) bool {
	for _, char := range input {
		if !unicode.IsLower(char) {
			return false
		}
	}
	return true
}
