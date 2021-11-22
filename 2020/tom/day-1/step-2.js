var fs = require('fs');
var file = fs.readFileSync('data.txt','utf8');
var dataArr = Array.from(file.split("\r\n"));

const target = 2020;

let index = []

for (i = 0 ; i < dataArr.length; i++) {
    for (j = 0; j < dataArr.length; j++) {
        if (dataArr.includes((target - dataArr[i] - dataArr[j]).toString(10)))  {
            index = [dataArr[i], dataArr[j], target - dataArr[i] - dataArr [j]];
            break;
        }
    }
}

console.log("numbers found:",index);
console.log("solution: ", index[0] * index[1] * index[2]);