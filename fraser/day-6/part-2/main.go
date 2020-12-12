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
	groupSize := 0
	groupAnsweredQuestions := make(map[string]int)

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
					// Remote any less than length of the group
					for key, count := range groupAnsweredQuestions {
						if count < groupSize {
							delete(groupAnsweredQuestions, key)
						}
					}
					// Add questions answered to result
					result = result + len(groupAnsweredQuestions)
					// Reset group
					groupSize = 0
					groupAnsweredQuestions = make(map[string]int)
				}
			}
		default:
			// Iterate over characters
			{
				for _, rune := range []rune(line) {
					character := string(rune)
					// If entry exists then iterate it
					if count, ok := groupAnsweredQuestions[character]; ok {
						groupAnsweredQuestions[character] = count + 1
						// If not set it to 1
					} else {
						groupAnsweredQuestions[character] = 1
					}
				}
				groupSize++
			}
		}
	}

	for key, count := range groupAnsweredQuestions {
		if count < groupSize {
			delete(groupAnsweredQuestions, key)
		}
	}
	// Add last line
	result = result + len(groupAnsweredQuestions)

	file.Close()

	println("Result", result)
}
