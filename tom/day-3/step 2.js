var fs = require('fs');
var file = fs.readFileSync('data.txt','utf8');
var dataArr = Array.from(file.split("\r\n"));

const rightIncrement = [1,3,5,7,1];
const downIncrement = [1,1,1,1,2];

let treeCounter = [0,0,0,0,0];

for (j = 0; j < downIncrement.length; j++) {   
    let currPlace = [0,0,0,0,0];
    for (i = 0; i < dataArr.length; i+= downIncrement[j]) {     
        const line = dataArr[i];
        if (currPlace[j] > line.length-1) {
            currPlace[j] = currPlace[j] - line.length;
        } 
        if (line.charAt(currPlace[j]) === "#") {
            treeCounter[j] += 1;
        }
        currPlace[j] += rightIncrement[j];
    }
}

console.log("Tree count: ", treeCounter);
solution = treeCounter.reduce((prevVal,currVal) => prevVal * currVal, 1);
console.log("Solution: ", solution);