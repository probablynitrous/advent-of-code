package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
	"flag"
	"time"
	"log"
)


func main() {
	testArg := flag.Bool("test", false, "")
	flag.Parse()
	var test bool = bool(*testArg)

	var data []string
	if test {
		data = readFile("../test.txt")
	} else {
		data = readFile("../input.txt")
	}
	
	step1 := step1(data)
	fmt.Println("Step 1: ",step1)
	step2 := step2(data)
	fmt.Println("Step 2: ",step2)
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
	var prioritySum int = 0
	for _,line := range data {
		var compartment1 string
		var compartment2 string

		compartment1 = line[:(len(line)/2)]
		compartment2 = line[(len(line)/2):]

		for _,char := range compartment1 {
			if strings.ContainsRune(compartment2, char) {	
				prioritySum += getPriority(char)		
				break
			}
		}
	}
	return	prioritySum
}

func step2(data []string) int {
	start := time.Now()
	var prioritySum int = 0
	var groups [][3]string
	var i int = 0
	var group [3]string

	//Group data
	for _,line := range data {
		group[i] = line
		if i == 2 {
			groups = append(groups, group)
			i = 0
		} else {
			i += 1
		}
	}

	//Compare values in groups
	for _,group := range groups {
		for _,char := range group[0] {
			if strings.ContainsRune(group[1], char) {	
				if strings.ContainsRune(group[2], char) {	
					prioritySum += getPriority(char)		
					break
				}
			}
		}
	}
	
	fmt.Printf("%s took %v\n", "step2", time.Since(start).Microseconds())
	return	prioritySum
}

func getPriority(c rune) int {
	if unicode.IsUpper(c) {
		return int(c) - 38
	} else {
		return  int(c) - 96
	}
}

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	log.Printf("%s took %s", name, elapsed)
}
