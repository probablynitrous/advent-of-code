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

dataArr.some(([_, value]) => {
  // Check that we don't have the remainder value in the list, as if we do that
  // won't result in a tuple answer.
  if (!data.get((target - value).toString(10))) {
    let remainder = target - value;

    // This makes it quadratic unfortunately, as we need to go back and check
    // all the values in the list as we might've missed some given our current
    // index.
    for (let i = 0; i < dataArr.length; i++) {
      const num2 = dataArr[i][1];
      const num3 = remainder - num2;

      // Check that the value is greater than 0, is present in the list AND
      // that the three numbers add up to the target (for sanity).
      if (
        num3 > 0 &&
        data.get(num3.toString(10)) &&
        value + num2 + num3 === target
      ) {
        // Set the values and escape early out of the loop.
        values = [value, num2, num3];
        return true;
      }
    }
  }
});

console.log("time taken: ", performance.now() - time0);
console.log("values: ", values);

console.log(
  "solution: ",
  values.reduce((val, acc) => val * acc, 1)
);
