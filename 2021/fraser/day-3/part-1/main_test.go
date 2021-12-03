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
		Instruction{0, 1},
		Instruction{0, 1},
		Instruction{2, 3},
	}

	actual := instructions.getMostCommonInIndex(0)
	expected := 0
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
	actual = instructions.getMostCommonInIndex(1)
	expected = 1
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetleastCommonIndex(t *testing.T) {
	instructions := Instructions{
		Instruction{0, 1},
		Instruction{0, 1},
		Instruction{2, 3},
	}

	actual := instructions.getLeastCommonInIndex(0)
	expected := 2
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
	actual = instructions.getLeastCommonInIndex(1)
	expected = 3
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetGammaRate(t *testing.T) {
	instructions := getInstructions("test.txt")
	actual := getGammaRate(instructions)
	expected := 22
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetEpsilonRate(t *testing.T) {
	instructions := getInstructions("test.txt")
	actual := getEpsilonRate(instructions)
	expected := 9
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestInstructionsGetPowerConsumption(t *testing.T) {
	instructions := getInstructions("test.txt")
	actual := getPowerConsumption(instructions)
	expected := 198
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
