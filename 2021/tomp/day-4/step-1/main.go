package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"strconv"
)
type Results struct{
	bingoNumber int
	unmarked int
}

func main() {
	data := readFile("test.txt")
	numbersDrawn := getNumbersDrawn(data)
	boards := buildBoards(data)
	winningValues := playBingo(numbersDrawn, boards)
	fmt.Println(winningValues.unmarked * winningValues.bingoNumber)
}

func readFile(fileName string)[]string {
	data, _ := os.Open(fileName)
	scanner := bufio.NewScanner(data)

	var dataLines []string
	for scanner.Scan() {
		line := scanner.Text()
		dataLines = append(dataLines, line)
	}
	return dataLines
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
	fmt.Println(board)
	return boards
}

func playBingo(numbersDrawn []string, boards map[int][][]string)Results{

	columnsMarked := make([][5]int, len(boards)+1)
	rowsMarked := make([][5]int, len(boards)+1)
	unmarked := make(map[int]int)

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
				colIndex := Find(boardLine, value)
				if colIndex < len(boardLine) {
					rowsMarked[boardIndex][rowIndex] += 1
					columnsMarked[boardIndex][colIndex] += 1

					valueInt,_ := strconv.Atoi(value)
					unmarked[boardIndex] -= valueInt

					if rowsMarked[boardIndex][rowIndex] == 5 || columnsMarked[boardIndex][colIndex] == 5  {
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

	return Results{bingoNumber,unmarked[bingoBoard]}
}