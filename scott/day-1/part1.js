const { readFileSync } = require("fs");
const { performance } = require("perf_hooks");

// Read in the file into a Map for quick access
const data = new Map(
  readFileSync("./input.txt", "utf8")
    .split("\n")
    .sort((a, b) => a - b)
    .map((val) => [val, parseInt(val, 10)])
);

// Dump it into an Array so we can call `.some` for quick breakout
const dataArr = Array.from(data);

const target = 2020;

let values = [];
let time0 = performance.now();

// Iterate through the array of values
// If we find a solution to our equations in the map, then set those values
// and break out quickly.
dataArr.some(([_, numberValue]) => {
  if (data.get((target - numberValue).toString(10))) {
    values = [numberValue, target - numberValue];
    return true;
  }
});

console.log("time taken: ", performance.now() - time0);
console.log("values: ", values);

console.log(
  "solution: ",
  values.reduce((val, acc) => val * acc, 1)
);
