const fs = require('fs');
const file = fs.readFileSync("data.txt","utf8");
const dataArr = file.split("\r\n");

let currentSample = [];
currentSample = dataArr.slice(0,25);
let solution = 0;
for (i = 25; i < dataArr.length; i++) {
    if (valuesSum(dataArr[i])) {
        solution = dataArr[i];
        break;
    }
    currentSample.push(dataArr[i]);
    currentSample.shift();
}

console.log(solution)

function valuesSum(targetValue) {
    for (j = 0; j < currentSample.length; j++) {
        let value = currentSample[j];
        let currIndex = currentSample.indexOf(value);
        let newSample = currentSample.slice();
        newSample.splice(currIndex,1);
        for (k =0; k < newSample.length; k++) {
            let result = parseInt(value) + parseInt(newSample[k]);
            if (result === parseInt(targetValue)) {
                return false;
            }
        };
    };
    return true
}