package main

import (
	"bufio"
	"errors"
	"log"
	"os"
)

type space struct {
	seat     bool
	occupied bool
}

type boat [][]space

func (boat boat) String() string {
	s := ""
	for _, row := range boat {
		for _, space := range row {
			if space.seat {
				if space.occupied {
					s += "#"
				} else {
					s += "L"
				}
			} else {
				s += "."
			}
		}
		s += "\n"
	}
	return s
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	var boat boat

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
		var err error
		boat, err = tick(boat)
		if err != nil {
			log.Print(err.Error())
			break
		}
	}

	// Calculate total occupied
	result := 0

	for y := range boat {
		for x := range boat[y] {
			if boat[y][x].occupied {
				result++
			}
		}
	}

	log.Printf("Result %v", result)
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

func tick(currentBoat boat) (boat, error) {
	changes := 0

	newBoat := make(boat, len(currentBoat))

	for i := range currentBoat {
		newBoat[i] = make([]space, len(currentBoat[i]))
		copy(newBoat[i], currentBoat[i])
	}

	for y := range currentBoat {
		for x := range currentBoat[y] {
			// If it's not a seat then ignore it
			if !currentBoat[y][x].seat {
				continue
			}

			// Second rule
			if currentBoat[y][x].occupied {
				if 4 < currentBoat.visibileOccupied(x, y) {
					newBoat[y][x].occupied = false
					changes++
				}
			} else {
				// First rule
				if currentBoat.visibileOccupied(x, y) < 1 {
					newBoat[y][x].occupied = true
					changes++
				}
			}
		}
	}

	if changes < 1 {
		return newBoat, errors.New("There were no changes")
	}
	return newBoat, nil
}

func (boat boat) visibileOccupied(x int, y int) int {
	count := 0
	// In Y axis
	// Positive
yPositive:
	for iy := y + 1; iy < len(boat); iy++ {
		if iy == y {
			continue
		}
		if boat[iy][x].seat {
			if boat[iy][x].occupied {
				count++
			}
			break yPositive
		}

	}

	// Negative
yNegative:
	for iy := y - 1; -1 < iy; iy-- {
		if iy == y {
			continue
		}
		if boat[iy][x].seat {
			if boat[iy][x].occupied {
				count++
			}
			break yNegative
		}

	}

	// In X axis
	// Positive
xPositive:
	for ix := x + 1; ix < len(boat[y]); ix++ {
		if ix == x {
			continue
		}
		if boat[y][ix].seat {
			if boat[y][ix].occupied {
				count++
			}
			break xPositive
		}

	}

	// Negative
xNegative:
	for ix := x - 1; -1 < ix; ix-- {
		if ix == x {
			continue
		}
		if boat[y][ix].seat {
			if boat[y][ix].occupied {
				count++
			}
			break xNegative
		}

	}

	// Diagonal Y+ X+
	i := 1
	for {
		x := x + i
		y := y + i
		if boat.outOfBounds(x, y) {
			break
		}
		if boat[y][x].seat {
			if boat[y][x].occupied {
				count++
			}
			break
		}

		i++
	}

	// Diagonal Y+ X-
	i = 1
	for {
		x := x - i
		y := y + i

		if boat.outOfBounds(x, y) {
			break
		}
		if boat[y][x].seat {
			if boat[y][x].occupied {
				count++
			}
			break
		}

		i++
	}

	// Diagonal Y- X+
	i = 1
	for {
		x := x + i
		y := y - i
		if boat.outOfBounds(x, y) {
			break
		}
		if boat[y][x].seat {
			if boat[y][x].occupied {
				count++
			}
			break
		}

		i++
	}

	// Diagonal Y- X-
	i = 1
	for {
		x := x - i
		y := y - i
		if boat.outOfBounds(x, y) {
			break
		}
		if boat[y][x].seat {
			if boat[y][x].occupied {
				count++
			}
			break
		}

		i++
	}
	return count
}

func (boat boat) outOfBounds(x int, y int) bool {
	if x < 0 {
		return true
	}
	if y < 0 {
		return true
	}

	if len(boat[0]) <= x {
		return true
	}

	if len(boat) <= y {
		return true
	}

	return false
}

func (boat boat) adjacentOccupied(x int, y int) int {
	count := 0
	for iy := y - 1; iy <= y+1; iy++ {
		for ix := x - 1; ix <= x+1; ix++ {
			// If not on the boat
			if boat.outOfBounds(ix, iy) {
				continue
			}
			// Ignore self
			if iy == y && ix == x {
				continue
			}
			if boat[iy][ix].occupied {
				count++
			}
		}
	}
	return count
}
