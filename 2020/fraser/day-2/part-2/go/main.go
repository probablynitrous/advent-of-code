package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type password struct {
	policy   policy
	password string
}

type policy struct {
	character string
	positions []int
}

// Return true if password is valid
func valid(password password) bool {
	// Count the matches
	matches := 0

	for _, position := range password.policy.positions {
		// Elves have no concept of 0 index
		position = position - 1

		// If position is outside of the string then it can't match
		if len(password.password) < position {
			break
		}

		// println(fmt.Sprintf("Checking if %v is at position %v in %v", password.policy.character, position+1, password.password))

		runes := []rune(password.password)

		// println(fmt.Sprintf("Rune %v is at position %v in %v", string(runes[position]), position+1, password.password))

		if string(runes[position]) == password.policy.character {
			matches++
		}
	}

	return matches == 1
}

const threads = 8

func main() {
	// Assume we can always open the file
	file, _ := os.Open("passwords.txt")
	count := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		regex := regexp.MustCompile(`^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$`)
		parts := regex.FindStringSubmatch(line)
		positionOne, _ := strconv.Atoi(parts[1])
		positionTwo, _ := strconv.Atoi(parts[2])
		password := password{
			policy: policy{
				character: parts[3],
				positions: []int{positionOne, positionTwo},
			},
			password: parts[4],
		}
		if valid(password) {
			count++
		}
	}

	defer file.Close()

	println(fmt.Sprintf("Have %v valid passwords", count))
}
