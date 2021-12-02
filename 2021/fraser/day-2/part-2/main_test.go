package main

import "testing"

func TestInstructionForward(t *testing.T) {
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

func TestInstructionUp(t *testing.T) {
	submarine := Submarine{}

	instruction := Instruction{
		direction: "up",
		value:     1,
	}

	submarine.move(instruction)

	if submarine.y != 0 {
		t.Errorf("Result %v but expected %v", submarine.x, 0)
	}
}

func TestInstructionDown(t *testing.T) {
	submarine := Submarine{}

	instruction := Instruction{
		direction: "down",
		value:     1,
	}

	submarine.move(instruction)

	if submarine.getDepth() != 0 {
		t.Errorf("Result %v but expected %v", submarine.getDepth(), 0)
	}
}

func TestInstructionDownForward(t *testing.T) {
	submarine := Submarine{}

	submarine.move(Instruction{
		direction: "down",
		value:     1,
	})

	submarine.move(Instruction{
		direction: "forward",
		value:     5,
	})

	if submarine.x != 5 {
		t.Errorf("Position x %v but expected %v", submarine.getDepth(), 5)
	}

	if submarine.getDepth() != 5 {
		t.Errorf("Depth %v but expected %v", submarine.getDepth(), 5)
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
	if submarine.getProduct() != 900 {
		t.Errorf("Result %v but expected %v", submarine.getProduct(), 900)
	}
}
