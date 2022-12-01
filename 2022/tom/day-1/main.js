const {readFileSync} = require('fs');

function main() {
  const data = readFile();
  const step1Result = step1(data);
  console.log(step1Result);
  const step2Result = step2(data);
  console.log(step2Result);
}

function readFile() {
  const contents = readFileSync('input.txt', 'utf-8');
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