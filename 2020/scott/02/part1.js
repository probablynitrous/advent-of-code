const { readFileSync } = require("fs");

// Read in the file into a Map for quick access
const file = readFileSync("./input.txt", "utf8")
  .split("\n")
  .map((line) => line.split(":"));

const validPasswords = [];

file.forEach((line) => {
  if (line.length !== 2) {
    return;
  }

  const [requirements, letter] = line[0].split(" ");
  const password = line[1].trim();
  const [min, max] = requirements.split("-");

  // If we don't have the letter present in the password then it's invalid.
  if (password.indexOf(letter) === -1) {
    return;
  }

  const regex = `^([^${letter}]*${letter}[^${letter}]*){${min},${max}}$`;
  if (password.match(regex)) {
    validPasswords.push(password);
  }
});

console.log("solution: ", validPasswords.length, " valid passwords");
