const {readFileSync} = require('fs');

function main() {
	var data;
	if (process.argv.slice(2).includes("test")) {
		data = readFile('../test.txt');
	} else {
		data = readFile('../input.txt');
	}
  const step1Result = step1(data);
  console.log("Step 1: ", step1Result);
  const step2Result = step2(data);
  console.log("Step 2: ", step2Result);
}

function readFile(fileName) {
  const contents = readFileSync(fileName, 'utf-8');
  const data = contents.split('\r\n');
  return data;
}

function step1(data)  {
	let topCalories = 0;
	let currentCalories = 0;
	data.forEach(function(line) {
		if (line === "") {
			if (topCalories < currentCalories) {
				topCalories = currentCalories;
			}
			currentCalories = 0;
		} else {
			let calories = parseInt(line);
			currentCalories += calories	;
		}
	
	})
	
	if (topCalories < currentCalories) {
		topCalories = currentCalories;
	}
	return topCalories;
}

function step2(data)  {
	let topCalories = [0,0,0];
	let currentCalories = 0;
	data.forEach(function(line) {
		if (line === "") {
			topCalories = updateTopCalories(topCalories,currentCalories);
			currentCalories = 0;
		} else {
			let calories = parseInt(line);
			currentCalories += calories	;
		}
	}) 
	
	topCalories = updateTopCalories(topCalories,currentCalories);

	var total = 0;
	topCalories.forEach(function(calories) {
		total += calories;
	})
	return total;
}

function updateTopCalories(topCalories, currentCalories){
  topCalories.sort(function(a, b){return a - b});
	if (currentCalories > topCalories[0]) {
		topCalories.shift();
		topCalories.push(currentCalories);
	}
	return topCalories;
}

main();