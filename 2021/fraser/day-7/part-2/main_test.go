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

func TestGetTotalFuelToXAll(t *testing.T) {
	crabs := getCrabs("test.txt")
	actual := crabs.getTotalFuelToX(2)
	expected := 206
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestGetTotalFuelToXSingleBackwards(t *testing.T) {
	crabs := Crabs{Crab{14}}
	actual := crabs.getTotalFuelToX(5)
	expected := 45
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
func TestGetTotalFuelToXSingleForwards(t *testing.T) {
	crabs := Crabs{Crab{1}}
	actual := crabs.getTotalFuelToX(5)
	expected := 10
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
func TestGetMeanReal(t *testing.T) {
	crabs := getCrabs("test.txt")
	actual := crabs.getMeanX()
	expected := 5
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestGetTotalFuelToMean(t *testing.T) {
	crabs := getCrabs("test.txt")
	actual := crabs.getTotalFuelToMean()
	expected := 168
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
