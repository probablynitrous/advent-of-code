package main

import (
	"bufio"
	"os"
	"sort"
)

const threads = 8

type boardingPass struct {
	row  int
	col  int
	seat int
	code string
}

type boardingPasses []boardingPass

func (boardingPasses boardingPasses) Less(i, j int) bool {
	return boardingPasses[i].seat < boardingPasses[j].seat
}

func (boardingPasses boardingPasses) Len() int {
	return len(boardingPasses)
}

func (boardingPasses boardingPasses) Swap(i, j int) {
	boardingPasses[i], boardingPasses[j] = boardingPasses[j], boardingPasses[i]
}

func newBoardingPass(s string) boardingPass {
	rowLower := 0
	rowUpper := 127
	colLower := 0
	colUpper := 7
	var col int
	var row int

	for _, rune := range []rune(s) {
		character := string(rune)
		switch character {
		case "F":
			{
				// Lower half
				rowUpper = (rowLower + rowUpper) / 2
				row = rowUpper
			}
		case "B":
			{
				// Upper half
				rowLower = (rowLower + rowUpper) / 2
				rowLower++
				row = rowLower
			}
		case "L":
			{
				// Lower half
				colUpper = (colLower + colUpper) / 2
				col = colUpper
			}
		case "R":
			{
				// Upper half
				colLower = (colLower + colUpper) / 2
				colLower++
				col = colLower
			}
		}
	}

	seat := (row * 8) + col

	return boardingPass{
		col:  col,
		row:  row,
		code: s,
		seat: seat,
	}
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	passes := boardingPasses{}

	// For each line
	for scanner.Scan() {
		passes = append(passes, newBoardingPass(scanner.Text()))
	}

	file.Close()

	sort.Sort(passes)
	seats := []int{}
	for _, pass := range passes {
		seats = append(seats, pass.seat)
	}

	sort.Ints(seats)

	for i := 0; i < len(seats)-1; i++ {
		currentSeat := seats[i]
		nextSeat := seats[i+1]
		if nextSeat == currentSeat+2 {
			println("Answer", currentSeat+1)
			return
		}
	}
}
