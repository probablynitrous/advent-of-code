package main

import "testing"

func TestMoveForward(t *testing.T) {
	submarine := Submarine{}

	instruction := Instruction{
		direction: "forward",
		value:     1,
	}

	submarine.move(instruction)

	if submarine.x != 1 {
		t.Errorf("Result %v but expected %v", submarine.x, 1)
	}
}

func TestMoveUp(t *testing.T) {
	submarine := Submarine{
		y: -1,
	}

	instruction := Instruction{
		direction: "up",
		value:     1,
	}

	submarine.move(instruction)

	if submarine.y != 0 {
		t.Errorf("Result %v but expected %v", submarine.x, 0)
	}
}

func TestMoveDown(t *testing.T) {
	submarine := Submarine{}

	instruction := Instruction{
		direction: "down",
		value:     1,
	}

	submarine.move(instruction)

	if submarine.getDepth() != 1 {
		t.Errorf("Result %v but expected %v", submarine.getDepth(), 1)
	}
}

func TestReadInstructions(t *testing.T) {
	instructions := readInstructions("test.txt")
	if instructions[0].direction != "forward" {
		t.Errorf("Result %v but expected %v", instructions[0].direction, "forward")
	}
	if instructions[0].value != 5 {
		t.Errorf("Result %v but expected %v", instructions[0].value, 5)
	}
}

func TestSolve(t *testing.T) {
	instructions := readInstructions("test.txt")
	submarine := Submarine{}
	for _, instruction := range instructions {
		submarine.move(instruction)
	}
	if submarine.getProduct() != 150 {
		t.Errorf("Result %v but expected %v", submarine.getProduct(), 150)
	}
}
