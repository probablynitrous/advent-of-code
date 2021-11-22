const fs = require('fs');
const file  = fs.readFileSync('data.txt','utf8');
const dataArray = file.split("\r\n");

let seats = [];

dataArray.forEach(line => {
    let rowNo = getRow(line);
    let columNo = getColumn(line);
    let idNo = (rowNo * 8) + columNo;   
    seats.push({row:rowNo, colum:columNo, id:idNo});
});    

console.log(Math.max.apply(Math, seats.map(function(o) { return o.id; })));

let idArr = [];

seats.forEach(seatInfo => {
    let idNo = seatInfo.id;
    idArr.push(idNo);
})

idArr.sort(function(a, b) {
  return a - b;
});

const mySeat = findGap(idArr);

console.log("My Seat is: ", mySeat);


function getColumn(line) {
    let currentBounds = findColumnBounds(line);
    if (currentBounds[0] === currentBounds[1])
    {
        return currentBounds[0];
    }
}

function getRow(line) {
    let currentBounds = findRowBounds(line);
    if (currentBounds[0] === currentBounds[1])
    {
        return currentBounds[0];
    }
}
function findRowBounds(line) {
    let currentBounds = [0,127];
    for (k = 0; k < 7; k++) {
        let direction = line.charAt(k);
        switch(direction) {
            case "F":
                currentBounds[1] = returnLower(currentBounds);
                break;
            case "B":
                currentBounds[0] = returnUpper(currentBounds);
                break;
        }  
    }
    return currentBounds;
}
function findColumnBounds(line) {
    let currentBounds = [0,7];
    for (j = 7; j < 10; j++) {
        let direction = line.charAt(j);
        switch(direction) {
            case "L":
                currentBounds[1] = returnLower(currentBounds);
                break;
            case "R":
                currentBounds[0] = returnUpper(currentBounds);
                break;
        }  
    }
    return currentBounds;
}
function returnLower(value) {
    return(Math.floor((value[1]-value[0])/2 + value[0]));
}
function returnUpper(value) {
    return(Math.ceil((value[1]-value[0])/2 + value[0]));
}

function findGap(idArr) {
    for (l = 1; l < idArr.length; l++) {
        if (idArr[l] - idArr[l-1] !== 1)
        {
            return idArr[l] - 1;
        }
    }
}