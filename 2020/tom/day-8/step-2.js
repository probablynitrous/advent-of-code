const fs = require('fs');
const file = fs.readFileSync("data.txt",'utf8');
const dataArr = file.split("\r\n");

let accumulator = 0;
let ranLines = [];
const regex = /(?<operation>[a-z]+) (?<direction>[-+])(?<argument>[0-9]+)/;

for (i = 0; i < dataArr.length;  i++) {
    let line = dataArr[i];
    let action = line.match(regex).groups;  
    if (action.operation === "jmp") {
        if (terminates(i + 1)) {
            break;
        };
    }
    let currLine = i;
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
    ranLines.push(currLine);
    
}

console.log(accumulator)

function terminates(j) {
    let termRanLines =  ranLines;
    let termAcc = accumulator;
    for (j; j < dataArr.length;  j++) {
        let line = dataArr[j];
        let currLine = j;
        let action = line.match(regex).groups;  
        if (termRanLines.indexOf(j) !== -1) {
            return false;
        }
        switch (action.operation) {
            case "acc":
                testAcc += parseInt(action.direction + action.argument);
                break;
            case "jmp":
                j += parseInt(action.direction + action.argument) - 1;
                break;
            default:
                break;
        }
        termRanLines.push(currLine);
    }   
    ranLines = termRanLines;
    accumulator = termAcc;
    return true;
}