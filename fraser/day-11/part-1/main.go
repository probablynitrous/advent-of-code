package main

import (
	"bufio"
	"log"
	"os"
)

type space struct {
	seat     bool
	occupied bool
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("test.txt")
	scanner := bufio.NewScanner(file)

	var boat [][]space

	// For each line
	for scanner.Scan() {
		boat = append(boat, []space{})
		y := len(boat) - 1
		line := scanner.Text()
		for _, rune := range []rune(line) {
			character := string(rune)
			space := newSpace(character)
			boat[y] = append(boat[y], space)
		}
	}

	file.Close()

	for {
		changes := tick(boat)
		if changes < 1 {
			break
		}
	}

	log.Printf("%v", boat)
}

func newSpace(character string) space {
	switch character {
	case "L":
		{
			return space{seat: true}
		}
	case "#":
		{
			return space{seat: true, occupied: true}
		}
	case ".":
		{
			return space{}
		}
	default:
		{
			log.Fatal("Unrecognised character")
		}
	}
	return space{}
}

func tick(boat [][]space) int {
	return 0
}
