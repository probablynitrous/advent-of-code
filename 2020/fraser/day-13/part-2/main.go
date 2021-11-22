package main

import (
	"bufio"
	"log"
	"os"
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

	period := 1
	base := 1
	for _, departure := range departures {
		iteration := 1
		for {
			t := base + (period * iteration)
			log.Printf("%v = %v * %v", t, iteration, base)
			if (departure.t+t)%departure.id == 0 {
				log.Printf("Matched %v at %v", departure.id, t)
				log.Printf("Base = %v * %v", base, departure.id)
				period = period * departure.id
				base = t
				iteration = 1
				break
			}
			iteration++
		}
	}

	log.Printf("Result %v", base)

	file.Close()
}
