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

func TestEntryCountSegmentFrequencies(t *testing.T) {
	entry := readEntries("test.txt")[0]
	frequencies := entry.getInputSegmentFrequencies()
	expected := "8"
	actual := frequencies["a"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
func TestGetLetterToSegmentMap(t *testing.T) {
	entry := readEntries("test.txt")[0]
	letterToSegmentMap := entry.getLetterToSegmentMap()
	expected := "0"
	actual := letterToSegmentMap["d"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "1"
	actual = letterToSegmentMap["e"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "2"
	actual = letterToSegmentMap["a"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "3"
	actual = letterToSegmentMap["f"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "4"
	actual = letterToSegmentMap["g"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "5"
	actual = letterToSegmentMap["b"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "6"
	actual = letterToSegmentMap["c"]
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestEntryGetFirstSignalOfLength(t *testing.T) {
	entry := lineToEntry("aaa aaa bb aaa | aa aaa aaa aaaa")
	expected := "bb"
	actual := entry.getFirstInputSignalOfLength(2)
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestGetSignalToNumberMap(t *testing.T) {
	entry := readEntries("test.txt")[0]
	expected := "1"
	actual := entry.signalToNumber("ab")
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "5"
	actual = entry.signalToNumber("cdfeb")
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

	expected = "3"
	actual = entry.signalToNumber("fcadb")
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestGetEntryOuput(t *testing.T) {
	entry := readEntries("test.txt")[0]
	expected := "5353"
	actual := entry.getOutput()
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
func TestGetEntryTotal(t *testing.T) {
	entries := readEntries("test-larger.txt")
	expected := 61229
	actual := entries.getTotal()
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
