
import sys

def main() : 
	data = []
	if 'test' in sys.argv:
		data = readFile('../test.txt')
	else:
		data = readFile('../input.txt')

	step1Result = step1(data)
	print ("Step 1: ", step1Result)
	step2Result = step2(data)
	print ("Step 2: ", step2Result)


def readFile(fileName):
  contents = open(fileName, "r")
  data = contents.read().split('\n')
  return data


def step1(data) :
	topCalories = 0
	currentCalories = 0
	for line in data:
		if line == "":
			if (topCalories < currentCalories):
				topCalories = currentCalories
			
			currentCalories = 0
		else: 
			calories = int(line)
			currentCalories += calories	
	
	if topCalories < currentCalories:
		topCalories = currentCalories

	return topCalories

def step2(data):
	topCalories = [0,0,0]
	currentCalories = 0
	for line in data:
		if line == "":
			topCalories = updateTopCalories(topCalories,currentCalories)
			currentCalories = 0
		else:
			calories = int(line)
			currentCalories += calories	
	
	topCalories = updateTopCalories(topCalories,currentCalories)

	total = 0
	for calories in topCalories:
		total += calories

	return total

def updateTopCalories(topCalories, currentCalories):
	topCalories.sort()
	if currentCalories > topCalories[0]:
		topCalories.pop(0)
		topCalories.append(currentCalories)

	return topCalories


if __name__ == '__main__':
    sys.exit(main())