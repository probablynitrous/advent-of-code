var fs = require('fs');
var file = fs.readFileSync('data.txt','utf8');
var dataArr = Array.from(file.split("\r\n"));

let pwcount = 0;
for (i = 0; i < dataArr.length; i++) {
    const line = dataArr[i];
    const regex = /^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)/;
    const result = line.match(regex);
    if (isVaild(result)) {
        pwcount += 1;
    }    
}

console.log("Total valid passwords: ", pwcount);

function isVaild(result) {
    min = result[1];
    max = result[2];
    critera = result[3];
    password = result[4];

    const regex = new RegExp("[^" + critera + "]", "g");
    const letterCount = password.replace(regex, "").length;

    if (letterCount >= min && letterCount <= max) {
        return true;
    }
    return false;
}

