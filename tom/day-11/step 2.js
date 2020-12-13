const fs = require('fs');
const file = fs.readFileSync('data.txt', 'utf8');
const dataArry = file.split("\r\n");

let lastSample = dataArry;
let complete = false;

for (i = 0; complete === false; i++) {
    let currentData = lastSample.slice();
    let newData = [];
    for (j = 0; j < currentData.length; j++) {
        let line = currentData[j];
        let newLine = "";
        for (k = 0; k < line.length; k++){
            newValue = getNewValue(line.charAt(k), currentData, k, j);
            newLine += newValue;
        }
        newData.push(newLine);
    }
    if (newData.join() === lastSample.join()) {
        complete = true;
    } else {
        lastSample = newData.slice();
    }
}

let solution = (lastSample.join().match(/#/g) || []).length;

console.log(solution)

function getNewValue(currValue, currentData, currPos, currLineIndex) {
    let surrounding = [];
    let ommitTop = (currLineIndex === 0);
    let ommitBottom = (currLineIndex === (currentData.length-1));
    let ommitLeft = (currPos === 0);
    let ommitRight = (currPos === (currentData[currLineIndex].length -1));

    if (!ommitTop) {
        if (!ommitLeft) {
            surrounding.push(findClosestNW(currentData, currPos, currLineIndex));
        }
        surrounding.push(findClosestN(currentData, currPos, currLineIndex));
        if (!ommitRight) {
            surrounding.push(findClosestNE(currentData, currPos, currLineIndex));
        }
    }

    if (!ommitBottom){
        if (!ommitLeft) {
            surrounding.push(findClosestSW(currentData, currPos, currLineIndex));
        }
        surrounding.push(findClosestS(currentData, currPos, currLineIndex));
        if (!ommitRight) {
            surrounding.push(findClosestSE(currentData, currPos, currLineIndex));
        }
    }
    
    if (!ommitLeft) {
        surrounding.push(findClosestW(currentData, currPos, currLineIndex));
    }
    if (!ommitRight) {
        surrounding.push(findClosestE(currentData, currPos, currLineIndex));
    }

    switch(currValue) {
        case "L": 
            if ((surrounding.join().match(/#/g) || []).length === 0) {
                return "#";
            }
            return "L";
        case "#":
            let emptySeatCounter = (surrounding.join().match(/#/g) || []).length;
            if (emptySeatCounter > 4) {
                return "L";
            } 
            return "#";
        default:
            return ".";
    }
}

function findClosestN(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (n = currLineIndex - 1; n >= 0; n--) {
        currValue = currentData[n].charAt(currPos);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;
}
function findClosestS(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (n = currLineIndex + 1; n < currentData.length; n++) {
        currValue = currentData[n].charAt(currPos);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;
}
function findClosestE(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (m = currPos + 1; m < currentData[currLineIndex].length; m++) {
        currValue = currentData[currLineIndex].charAt(m);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;
}
function findClosestW(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (m = currPos - 1; m >= 0; m--) {
        currValue = currentData[currLineIndex].charAt(m);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;
}
function findClosestNE(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (n = currLineIndex - 1, m = currPos + 1; n !== 0 && m < currentData[currLineIndex].length; n--, m++) {
        currValue = currentData[n].charAt(m);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;
}
function findClosestNW(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (n = currLineIndex - 1 , m = currPos-1; n >= 0 && m >= 0; n--, m--) {
        currValue = currentData[n].charAt(m);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;
}
function findClosestSE(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (n = currLineIndex + 1, m = currPos + 1; n < currentData.length && m < currentData[currLineIndex].length; n++, m++) {
        currValue = currentData[n].charAt(m);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;
}
function findClosestSW(currentData, currPos, currLineIndex) {
    let currValue = ".";
    for (n = currLineIndex + 1, m = currPos - 1; n < currentData.length && m >= 0; n++, m--) {
        currValue = currentData[n].charAt(m);
        if (currValue !== ".") {
            break;
        }
    }
    return currValue;6
}

