package main

import (
	"testing"
)

func TestCreateFish(t *testing.T) {
	fish := createFish(1)
	expected := 1
	actual := fish.age
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestShoalSize(t *testing.T) {
	shoal := Shoal{5: 2, 6: 2}
	actual := shoal.getSize()
	expected := 4
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestGetShoal(t *testing.T) {
	shoal := getShoal("test.txt")
	actual := shoal.getSize()
	expected := 5
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestShoalTicksShort(t *testing.T) {
	shoal := getShoal("test.txt")
	shoal = shoal.ticks(1)
	actual := shoal.getSize()
	expected := 5
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestShoalTicksMedium(t *testing.T) {
	shoal := getShoal("test.txt")
	shoal = shoal.ticks(18)
	actual := shoal.getSize()
	expected := 26
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
