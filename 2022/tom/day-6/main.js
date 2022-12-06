const { match } = require('assert');
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

function step1(data)
{
  const r = /^.*(.).*\1.*$/;
  const line = data[0];
  for (i = 0; i < line.length; i++) {
    const sample = line.slice(i, i+4);
    try {  
      match(sample,r);
    } catch {
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
    try {  
      match(sample,r);
    } catch {
      return i+14;
    } 
  }
  
  return "";
}

main()