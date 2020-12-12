var fs = require('fs');
var file = fs.readFileSync('data.txt','utf8');
var dataArr = Array.from(file.split("\r\n"));

const repeater = 31;
const increment = 3;
const tree = "#";

let treeCounter = 0;
let currPlace = 0;

const maxLen = increment * dataArr.length;

for (i = 0; i < dataArr.length; i++) {
    const line = dataArr[i].repeat(Math.ceil(maxLen/repeater));
    
    if (line.charAt(currPlace) === tree) {
        treeCounter += 1;
    }
    currPlace += increment;
}

console.log("Tree count: ", treeCounter);