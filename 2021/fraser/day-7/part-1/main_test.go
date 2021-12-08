package main

import (
	"testing"
)

func TestCreateFish(t *testing.T) {
	crab := createCrab(1)
	expected := 1
	actual := crab.x
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestReadTestFile(t *testing.T) {
	crabs := getCrabs("test.txt")
	actual := len(crabs)
	expected := 10
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
func TestGetMedian(t *testing.T) {
	crabs := Crabs{Crab{0}, Crab{5}, Crab{10}}
	actual := crabs.getMedianX()
	expected := 5
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestGetOptimumLocationMedian(t *testing.T) {
	crabs := getCrabs("test.txt")
	actual := crabs.getMedianX()
	expected := 2
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
func TestGetTotalFuelToMedian(t *testing.T) {
	crabs := getCrabs("test.txt")
	actual := crabs.getTotalFuelToMedian()
	expected := 37
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
