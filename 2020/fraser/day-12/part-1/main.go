package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
	"strconv"
)

type ship struct {
	angle int
	x     int
	y     int
}

type instruction struct {
	action string
	value  int
}

func move(s ship, i instruction) ship {
	switch i.action {
	case "N":
		s.y += i.value
	case "E":
		s.x += i.value
	case "S":
		s.y -= i.value
	case "W":
		s.x -= i.value
	case "L":
		s.angle -= i.value
	case "R":
		s.angle += i.value
	case "F":
		switch s.angle {
		case 0:
			s.y += i.value
		case 90:
			s.x += i.value
		case 180:
			s.y -= i.value
		case 270:
			s.x -= i.value
		default:
			log.Fatalf("Ship at invalid angle %v", s.angle)
		}
	default:
		log.Fatal("Unknown instruction")
	}

	for {
		if 0 <= s.angle && s.angle < 360 {
			break
		}

		if s.angle < 0 {
			s.angle += 360
		}

		if 360 <= s.angle {
			s.angle -= 360
		}
	}

	return s
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

	ship := ship{angle: 90, x: 0, y: 0}

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
