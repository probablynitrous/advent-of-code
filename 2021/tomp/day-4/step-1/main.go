package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"strconv"
)

func main() {
	data, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}

	step1(dataLines)
}

func step1(data []string) {
	numbersDrawn := getNumbersDrawn(data)
	
	boards := buildBoards(data)

	winningValues := playBingo(numbersDrawn, boards)


	fmt.Println(winningValues["unmarked"] * winningValues["bingoNumber"])
}

func getNumbersDrawn(data []string) []string {
	numbersDrawnLine := data[0]
	numbersDrawn := strings.Split(numbersDrawnLine, ",")
	return numbersDrawn
}

func Find(a []string, x string) int {
    for i, n := range a {
        if x == n {
            return i
        }
    }
    return len(a)
}

func totalLine(row []string)int{
	var total int
	for _,value := range row {
		valueInt,_ := strconv.Atoi(value)
		total += valueInt
	}
	return total
}

func buildBoards(data []string)map[int][][]string {
	var board []string
	boards := make(map[int][][]string)
	boardId := 0
	for i := 1; i < len(data); i++ {
		if data[i] == "" {
			boardId += 1
		} else {
			board = strings.Fields(data[i])
			boards[boardId] = append(boards[boardId], board)
		}
	}
	return boards
}

func playBingo(numbersDrawn []string, boards map[int][][]string)map[string]int{

	var occurance = map[int]map[string]map[int]int{}
	unmarked := make(map[int]int)

	//init maps
	for boardIndex := range boards {
		occurance[boardIndex] = map[string]map[int]int{}
		occurance[boardIndex]["rowMarked"] = map[int]int{}
		occurance[boardIndex]["colMarked"] = map[int]int{}		
	}

	for boardIndex,board := range boards {
		var boardTotal int
		for _, line := range board {
			boardTotal += totalLine(line)
		}
		unmarked[boardIndex] = boardTotal
	}

	var exit bool
	var bingoBoard int
	var bingoNumber int

	for _, value := range numbersDrawn {
		for boardIndex, currBoard := range boards {
			for rowIndex, boardLine := range currBoard {
				pos := Find(boardLine, value)
				if pos < len(boardLine) {
					occurance[boardIndex]["colMarked"][pos] += 1
					occurance[boardIndex]["rowMarked"][rowIndex] += 1

					valueInt,_ := strconv.Atoi(value)
					unmarked[boardIndex] -= valueInt

					if occurance[boardIndex]["colMarked"][pos] == 5 || occurance[boardIndex]["rowMarked"][rowIndex] == 5  {
						bingoNumber = valueInt
						bingoBoard = boardIndex
						exit = true
						break
					}
				}
			}
			if exit {
				break
			}
		}
		if exit {
			break
		}
	}

	returnValues := make(map[string]int)

	returnValues["bingoNumber"] = bingoNumber
	returnValues["unmarked"] = unmarked[bingoBoard]

	return returnValues
}