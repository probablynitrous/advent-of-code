package main

import (
	"bufio"
	"os"
)

// For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	result := 0

	groupAnsweredQuestions := make(map[string]struct{})

	// For each line
	for scanner.Scan() {
		// Line availbile here
		line := scanner.Text()

		switch line {
		case "":
			// Reset the group and add length to the result
			{
				// If new group
				if line == "" {
					// Add questions answered to result
					result = result + len(groupAnsweredQuestions)
					// Reset group
					groupAnsweredQuestions = make(map[string]struct{})
				}
			}
		default:
			// Iterate over characters
			{
				for _, rune := range []rune(line) {
					character := string(rune)
					groupAnsweredQuestions[character] = struct{}{}
				}
			}
		}
	}

	// Add last line
	result = result + len(groupAnsweredQuestions)

	file.Close()

	println("Result", result)
}
