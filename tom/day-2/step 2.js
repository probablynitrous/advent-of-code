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

    const pos1 = (password.charAt(parseInt(min) - 1 ));
    const pos2 = (password.charAt(parseInt(max) - 1));

    const pos1Valid = pos1 === critera;
    const pos2Valid = pos2 === critera;

    if (pos1Valid ^ pos2Valid) {
        return true;
    }
    return false;
}