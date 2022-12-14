const { match } = require('assert');
const {readFileSync} = require('fs');

async function main() {
	var data;
	if (process.argv.slice(2).includes("test")) {
		data = readFile('../test.txt');
	} else {
		data = readFile('../input.txt');
	}
  const step1Result = await step1(data);
  console.log("Step 1: ", step1Result);
  const step2Result = step2(data);
  console.log("Step 2: ", step2Result);
}

function readFile(fileName) {
  const contents = readFileSync(fileName, 'utf-8');
  const data = contents.split('\r\n\r\n');
  return data;
}


function step1(data)  {
	total = 0
	for ([j,pairText] of data.entries()) {
		pair = pairText.split('\r\n')
		let left = JSON.parse(pair[0])
		let right = JSON.parse(pair[1])

		if (compareArray(left, right)) {	
			total += j + 1
		}
		
	}
	return total	
}


function step2(data)  {
	total = 0
	let pairs = []
	for (pairText of data) {
		pair = pairText.split('\r\n')
		let left = JSON.parse(pair[0])
		let right = JSON.parse(pair[1])
		pairs.push(left)		
		pairs.push(right)		
	}

	pairs.push([[6]])
	pairs.push([[2]])

	let sortedPairs = pairs.sort((a,b) => compareArray(b,a) - compareArray(a,b))

	let multiplier = []

	for ([i,line] of sortedPairs.entries()) {
		if (['[[6]]','[[2]]'].includes(JSON.stringify(line))) {
			multiplier.push(i+1)
		}
	}

	return multiplier.reduce( (a, b) => a * b)	
}

function compareArray(left , right) {
	if (Number.isInteger(left) && Number.isInteger(right)) {
		if(left > right) {
			return false
		} 
		if (left < right) {
			return true
		}		
		return null
	}

	if (Array.isArray(left) && Array.isArray(right)) {
		let mini = Math.min(left.length, right.length)
		for (let i = 0; i < mini; i++) {
			const result = compareArray(left[i], right[i])
			if (result != null) {
				return result
			}
		}
		
		if (left.length > right.length) {
			return false
		}
		if (left.length < right.length) {
			return true
		}
	}

	if (Array.isArray(left) && Number.isInteger(right))
	{
		return compareArray(left, [right])
	}

	if (Array.isArray(right) && Number.isInteger(left))
	{
		return compareArray([left], right)
	}
}

main()