const { match } = require('assert');
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

function step1(data)
{
  const r = /^.*(.).*\1.*$/;
  const line = data[0];
  for (i = 0; i < line.length; i++) {
    const sample = line.slice(i, i+4);
    if (!sample.match(r)) {
      return i+4;
    } 
  }
  
  return "";
}

function step2(data)
{
  const r = /^.*(.).*\1.*$/;
  const line = data[0];
  for (i = 0; i < line.length; i++) {
    const sample = line.slice(i, i+14);
    if (!sample.match(r)) {
      return i+14;
    } 
  }
  
  return "";
}

main()