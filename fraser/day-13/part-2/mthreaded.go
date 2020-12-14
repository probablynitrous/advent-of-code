package main

import (
	"bufio"
	"log"
	"os"
	"runtime"
	"sort"
	"strconv"
	"strings"
)

type departure struct {
	id int
	t  int
}

type departures []departure

func (d departures) Len() int           { return len(d) }
func (d departures) Swap(i, j int)      { d[i], d[j] = d[j], d[i] }
func (d departures) Less(i, j int) bool { return d[j].id < d[i].id }

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	var departures departures

	index := 0
	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		if 0 < index {
			for t, s := range strings.Split(line, ",") {
				if s != "x" {
					id, _ := strconv.Atoi(s)
					departures = append(departures, departure{id: id, t: t})
				}
			}
		}
		index++
	}

	log.Printf("Departures %v", departures)
	sort.Sort(departures)
	log.Printf("Departures %v", departures)

	resultChan := make(chan int)

	threads := runtime.NumCPU()

	log.Printf("Running on %v threads", threads)

	// For x threads
	for i := 1; i < threads+1; i++ {
		go func(threadIndex int, resultChan chan int) {
			iteration := threadIndex
		iteration:
			for {
				t := iteration * departures[0].id

				relativeT := departures[0].t

				for index := 1; index < len(departures); index++ {
					if 0 < (t+(departures[index].t-relativeT))%(departures[index].id) {
						iteration += threads
						continue iteration
					}
				}

				log.Printf("Found result %v", t-relativeT)
				resultChan <- t - relativeT
			}
		}(i, resultChan)
	}

	result := <-resultChan

	log.Printf("Result %v", result)

	file.Close()
}
