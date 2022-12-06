package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
)


func main() {
	data := readFile("../test.txt")
	step1 := step1(data)
	fmt.Println(step1)
	step2 := step2(data)
	fmt.Println(step2)
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

func step1(data []string) int {
	r,err := regexp.MatchString(`^.*(.).*\1.*$`,"abcdd")
	fmt.Println(r)
	fmt.Println(err)
	return 0
}

func step2(data []string) int {
	
	return 0
}
