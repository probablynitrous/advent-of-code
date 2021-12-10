package main

import (
	"testing"
)

func TestLineToInput(t *testing.T) {
	line := "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
	expected := "be"
	actual := lineToEntry(line).inputs[0]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestCountOutputSignalsOfLength(t *testing.T) {
	entry := lineToEntry(" | aa aaa aaa aaaa")
	expected := 2
	actual := entry.countOutputSignalsOfLength(3)
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestCountOutputSignalsOfLengths(t *testing.T) {
	entry := lineToEntry(" | aa aaa aaa aaaa")
	expected := 3
	actual := entry.countOutputSignalsOfLengths([]int{2, 3})
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
func TestReadFile(t *testing.T) {
	entries := readEntries("test.txt")
	expected := "acedgfb"
	actual := entries[0].inputs[0]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "cdfeb"
	actual = entries[0].outputs[0]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestCountOutputSignalsOfLengthsActual(t *testing.T) {
	entries := readEntries("test-larger.txt")
	expected := 26
	actual := entries.countOutputSignalsOfLengths([]int{2, 3, 4, 7})
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

}
