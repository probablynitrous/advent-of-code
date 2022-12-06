package main

import (
	"bufio"
	"fmt"
	"os"
	"flag"
)


type Game struct {
	me int
	op int
}

//6
//Rock wins when 1-3 = -2  
//Paper wins when 2-1 = 1
//Scissors wins when 3-2 = 1

//3
//Draw = 0TY

//0
//Rock loses when 1-2 = -1
//Paper loses when 2-3 = -1
//Scissors loses when 3-1 = 2


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

func step1(data []string)int {
	var total int = 0
	for _,line := range data {
		var game Game
		switch line[0:1] {
			case "A":
				game.op = 1;
				break
			case "B":
				game.op = 2;
				break
			case"C":
				game.op = 3;
				break
		}
		
		switch line[2:3] {
			case "X":
				game.me = 1;
				break
			case "Y":
				game.me = 2;
				break
			case"Z":
				game.me = 3;
				break
		}

		total += getScore(game)
	}	
	return total
}

func step2(data []string)int {
	var total int = 0
	for _,line := range data {
		var game Game
		switch line[0:1] {
			case "A":
				game.op = 1;
				break
			case "B":
				game.op = 2;
				break
			case"C":
				game.op = 3;
				break
		}
		
		switch line[2:3] {
			case "X":
				switch game.op {
					case 1:
						game.me = 3
					case 2:
						game.me = 1
					case 3:
						game.me = 2

				}
				break
			case "Y":
				switch game.op {
					case 1:
						game.me = 1
					case 2:
						game.me = 2
					case 3:
						game.me = 3

				}
				break
			case"Z":
				switch game.op {
					case 1:
						game.me = 2
					case 2:
						game.me = 3
					case 3:
						game.me = 1

				}
				break
		}

		total += getScore(game)
	}	
	return total
}

func getScore(game Game)int {
	switch game.me {
		case 1:
			switch game.me - game.op {
				case -2:
					return 7
				case -1:
					return 1
				case 0:
					return 4
			}
		case 2:
			switch game.me - game.op {
				case 1:
					return 8
				case -1:
					return 2
				case 0:
					return 5
			}
		case 3:
			switch game.me - game.op {
				case 1:
					return 9
				case 2:
					return 3
				case 0:
					return 6
			}
	}
	return 0
}