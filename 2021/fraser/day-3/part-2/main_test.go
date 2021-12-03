package main

import (
	"testing"
)

func TestRunesToInts(t *testing.T) {
	runes := []rune{'0', '1'}
	ints := runesToInts(runes)
	actual := ints[0]
	expected := 0
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
	actual = ints[1]
	expected = 1
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetMostCommonIndex(t *testing.T) {
	instructions := Instructions{
		Instruction{0, 0},
		Instruction{1, 0},
	}

	actual := instructions.getMostCommonInIndex(0)
	expected := 1
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
	actual = instructions.getMostCommonInIndex(1)
	expected = 0
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetleastCommonIndex(t *testing.T) {
	instructions := Instructions{
		Instruction{1, 0},
		Instruction{0, 0},
	}

	actual := instructions.getLeastCommonInIndex(0)
	expected := 0
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
	actual = instructions.getLeastCommonInIndex(1)
	expected = 0
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetOxygenGeneratorRating(t *testing.T) {
	instructions := getInstructions("test.txt")
	actual := getOxygenGeneratorRating(instructions)
	expected := 23
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetCO2ScrubberRating(t *testing.T) {
	instructions := getInstructions("test.txt")
	actual := getCO2ScrubberRating(instructions)
	expected := 10
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetLifeSupportRating(t *testing.T) {
	instructions := getInstructions("test.txt")
	actual := getLifeSupportRating(instructions)
	expected := 230
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
