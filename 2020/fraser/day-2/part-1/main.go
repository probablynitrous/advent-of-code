package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type password struct {
	policy   policy
	password string
}

type policy struct {
	character string
	min       int
	max       int
}

// Return true if password is valid
func valid(password password) bool {
	count := strings.Count(password.password, password.policy.character)
	if count < password.policy.min {
		return false
	}
	if password.policy.max < count {
		return false
	}
	return true
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
		min, _ := strconv.Atoi(parts[1])
		max, _ := strconv.Atoi(parts[2])
		password := password{
			policy: policy{
				character: parts[3],
				min:       min,
				max:       max,
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
