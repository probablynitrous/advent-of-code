const fs = require('fs');
const file  = fs.readFileSync('data.txt','utf8');
const dataArray = file.split("\r\n\r\n");
const values = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
let validCounter = 0;

dataArray.forEach(element => {
    let passports = getPasswordObj(element);
    if (isValid(passports)) {
        validCounter += 1;
    }
});

console.log("Valid passports: ", validCounter);

function getPasswordObj(rawLine) {
    let result = {};
    var line = rawLine.replace(/[\r\n]+/g," ");
    line.split(" ").forEach(function(objStr) {
        objArr = objStr.split(":");
        result[objArr[0]] = objArr[1];
    });
    return result;
}

function isValid(passports) {
    return values.every(function(id) {
        return id in passports;
    })
}