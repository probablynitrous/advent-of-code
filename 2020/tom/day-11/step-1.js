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
            surrounding.push(currentData[currLineIndex - 1].charAt(currPos-1));
        }
        surrounding.push(currentData[currLineIndex - 1].charAt(currPos));
        if (!ommitRight) {
            surrounding.push(currentData[currLineIndex - 1].charAt(currPos+1));
        }
    }

    if (!ommitBottom){
        if (!ommitLeft) {
            surrounding.push(currentData[currLineIndex + 1].charAt(currPos-1));
        }
        surrounding.push(currentData[currLineIndex + 1].charAt(currPos));
        if (!ommitRight) {
            surrounding.push(currentData[currLineIndex + 1].charAt(currPos+1));
        }
    }
    
    if (!ommitLeft) {
        surrounding.push(currentData[currLineIndex].charAt(currPos-1));
    }
    if (!ommitRight) {
        surrounding.push(currentData[currLineIndex].charAt(currPos+1));
    }

    switch(currValue) {
        case "L": 
            if ((surrounding.join().match(/#/g) || []).length === 0) {
                return  "#";
            }
            return "L";
        case "#":
            let emptySeatCounter = (surrounding.join().match(/#/g) || []).length;
            if (emptySeatCounter > 3) {
                return "L";
            } 
            return "#";
        default:
            return ".";
    }
}
