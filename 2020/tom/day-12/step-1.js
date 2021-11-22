const fs = require('fs');
const file = fs.readFileSync("data.txt", "utf8");
const dataArr = file.split("\r\n");

let currPos = {North:0,South:0,East:0,West:0};
let currDir = "East";

dataArr.forEach(line => {
    const regex = /(?<action>[A-Z])(?<value>[0-9]+)/
    const direction = line.match(regex).groups;
    switch (direction.action) {
        case "N":
            currPos.North += parseInt(direction.value);
            break;
        case "S":
            currPos.South += parseInt(direction.value);
            break;
        case "E":
            currPos.East += parseInt(direction.value);
            break;
        case "W":
            currPos.West += parseInt(direction.value);
            break;
        case "F":
            currPos[currDir] += parseInt(direction.value);
            break;
        default:
            currDir = setDir(direction.value,direction.action);
    }
});

const solution = Math.abs(currPos.North - currPos.South) + Math.abs(currPos.East - currPos.West)

console.log(solution);

function setDir(moveBy, direction) {
    let degree = 0;
    switch(currDir)    {
        case "North":
            degree = 0;
            break;
        case "East":
            degree = 90;
            break;
        case "South":
            degree = 180;
            break;
        case "West":
            degree = 270;
            break;
    }
    if (direction === "R") {
        degree += parseInt(moveBy);
    } else {
        degree -= parseInt(moveBy);
    }

    if (degree > 360) {
        degree -= 360;
    }
    if (degree < -360) {
        degree += 360;
    }
    switch(degree){
        case 0:
        case 360:
            return "North";
        case 90:
        case -270:
            return "East";
        case 180:
        case -180:
            return "South";
        case 270: 
        case -90:
            return "West";
    }
}