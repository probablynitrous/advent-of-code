const fs = require('fs');
const file = fs.readFileSync("data.txt","utf8");
const dataArr = file.split("\r\n");

let currentSample = [];
currentSample = dataArr.slice(0,25);
let target = 0;
for (i = 25; i < dataArr.length; i++) {
    if (valuesSum(dataArr[i])) {
        target = dataArr[i];
        break;
    }
    currentSample.push(dataArr[i]);
    currentSample.shift();
}
let solution = 0;
for (m = 0; m < dataArr.length; m++) {
    const solutionArr = totalLine(dataArr[m]);
    if (solutionArr.length !== 0) {
        solution = Math.min.apply(null, solutionArr) + Math.max.apply(null, solutionArr);
        break;
    }
};

console.log(solution);

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

function totalLine(line) {
    const startFrom = dataArr.indexOf(line);
    let currValue = 0;
    let currSample = [];
    for (l = startFrom; currValue <= target; l++) {
        currValue += parseInt(dataArr[l]);
        currSample.push(dataArr[l]);
        if (currValue === parseInt(target)) {
            return currSample;
        }
    }
    return [];
}