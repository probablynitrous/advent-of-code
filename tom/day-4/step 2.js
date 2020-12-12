const fs = require('fs');
const file  = fs.readFileSync('data.txt','utf8');
const dataArray = file.split("\r\n\r\n");
const values = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
let validCounter = 0;
let number  = 0;

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
        if (id in passports) {
            switch(id) {
                case values[0]:
                    return checkbyr(passports[id]);                    
                case values[1]:                    
                    return checkiyr(passports[id]);
                case values[2]:                    
                    return checkeyr(passports[id]);
                case values[3]:                    
                    return checkhgt(passports[id]);
                case values[4]:                    
                    return checkhcl(passports[id]);
                case values[5]:
                    return checkecl(passports[id]);
                case values[6]:
                    return checkpid(passports[id]);             
            }
        }
    })
}

function checkbyr(value) {
    return (value >= 1920 && value <= 2002)
}
function checkiyr(value) {
    return (value >= 2010 && value <= 2020);
}
function checkeyr(value) {
    return (value >= 2020 && value <= 2030)
}
function checkhgt(value) {
    switch(value.slice(-2)) {
        case "in":
            number = value.replace("in", "");
            return (number >= 59 && number <= 76);
        case "cm":
            number = value.replace("cm", "");
            return (number >= 150 && number <= 193);
        default:
            return false;
    }
}
function checkhcl(value) {
    if (value.charAt(0) != "#") {
        return false;
    }
    if (value.length === 7) {
        return true;
    }
}
function checkecl(value) {
    let valid = false;
    colourOptions = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    colourOptions.forEach(colour => {
        if (value === colour){
            valid = true;
        }
    })
    return valid;
}
function checkpid(value) {
    if (!isNaN(value)) {
        return(value.length === 9)
    }
}