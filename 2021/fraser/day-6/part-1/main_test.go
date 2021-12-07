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

func TestCreateNewFish(t *testing.T) {
	fish := createNewFish()
	expected := 8
	actual := fish.age
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestFishTickAge(t *testing.T) {
	fish := createFish(0)
	fish, _ = fish.tick()
	expected := 6
	actual := fish.age
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}

}
func TestFishTickCreate(t *testing.T) {
	fish := createFish(0)
	_, createNew := fish.tick()
	expected := true
	actual := createNew
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestFishesTick(t *testing.T) {
	fishes := Fishes{createFish(0)}
	fishes = fishes.tick()
	actual := len(fishes)
	expected := 2
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestReadTestFile(t *testing.T) {
	fishes := getFishes("test.txt")
	actual := len(fishes)
	expected := 5
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestFishesTicksShort(t *testing.T) {
	fishes := getFishes("test.txt")
	fishes = fishes.ticks(1)
	actual := len(fishes)
	expected := 5
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestFishesTicksMedium(t *testing.T) {
	fishes := getFishes("test.txt")
	fishes = fishes.ticks(18)
	actual := len(fishes)
	expected := 26
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}

func TestFishesTicksLong(t *testing.T) {
	fishes := getFishes("test.txt")
	fishes = fishes.ticks(80)
	actual := len(fishes)
	expected := 5934
	if actual != expected {
		t.Errorf("Result %v but expected %v", actual, expected)
	}
}
