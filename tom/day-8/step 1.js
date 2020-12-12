const fs = require('fs');
const file = fs.readFileSync("data.txt",'utf8');
const dataArr = file.split("\r\n");

let accumulator = 0;
let ranLines = [];

for (i = 0;ranLines.indexOf(i) === -1;  i++) {
    let line = dataArr[i];
    ranLines.push(i);
    let regex = /(?<operation>[a-z]+) (?<direction>[-+])(?<argument>[0-9]+)/;
    let action = line.match(regex).groups;  
    switch (action.operation) {
        case "acc":
            accumulator += parseInt(action.direction + action.argument);
            break;
        case "jmp":
            i += parseInt(action.direction + action.argument) - 1;
            break;
        default:
            break;
    }
}

console.log(accumulator);