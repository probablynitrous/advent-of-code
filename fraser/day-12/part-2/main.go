package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
	"strconv"
)

type waypoint struct {
	dx int
	dy int
}

type ship struct {
	x        int
	y        int
	waypoint waypoint
}

type instruction struct {
	action string
	value  int
}

// Waypoint moves relative to ship
func move(s ship, i instruction) ship {
	switch i.action {
	case "N":
		s.waypoint.dy += i.value
	case "E":
		s.waypoint.dx += i.value
	case "S":
		s.waypoint.dy -= i.value
	case "W":
		s.waypoint.dx -= i.value
	case "L":
		for counter := 0; counter < i.value/90; counter++ {
			s.waypoint = swap(s.waypoint)
			s.waypoint.dx = -s.waypoint.dx
		}
	case "R":
		for counter := 0; counter < i.value/90; counter++ {
			s.waypoint = swap(s.waypoint)
			s.waypoint.dy = -s.waypoint.dy
		}
	case "F":
		// Move the ship
		s.y += s.waypoint.dy * i.value
		s.x += s.waypoint.dx * i.value
	default:
		log.Fatal("Unknown instruction")
	}

	return s
}

func swap(w waypoint) waypoint {
	placeholder := w.dx
	w.dx = w.dy
	w.dy = placeholder
	return w
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	instructions := []instruction{}

	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		regex := regexp.MustCompile("^([a-zA-Z])([0-9]+)$")
		matches := regex.FindStringSubmatch(line)
		value, _ := strconv.Atoi(matches[2])

		instruction := instruction{action: matches[1], value: value}
		instructions = append(instructions, instruction)
	}

	file.Close()

	ship := ship{x: 0, y: 0, waypoint: waypoint{dx: 10, dy: 1}}

	for _, instruction := range instructions {
		ship = move(ship, instruction)
	}

	log.Printf("Ship %+v", ship)
	log.Printf("Result %v", abs(ship.x)+abs(ship.y))
}

// Abs returns the absolute value of x.
func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
