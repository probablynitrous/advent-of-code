const { dir } = require('console');
const fs = require('fs');
const file = fs.readFileSync("data.txt", "utf8");
const dataArr = file.split("\r\n");

let currPos = {North:0,South:0,East:0,West:0};
let waypoint = {North:1,South:0,East:10,West:0};

dataArr.forEach(line => {
    const regex = /(?<action>[A-Z])(?<value>[0-9]+)/
    const direction = line.match(regex).groups;
    switch (direction.action) {
        case "N":
            waypoint.North += parseInt(direction.value);
            break;
        case "S":
            waypoint.South += parseInt(direction.value);
            break;
        case "E":
            waypoint.East += parseInt(direction.value);
            break;
        case "W":
            waypoint.West += parseInt(direction.value);
            break;
        case "F":
            let yValue = waypoint.North - waypoint.South;
            let xValue = waypoint.East - waypoint.West;
            if (yValue > 0) {
                currPos.North += Math.abs(yValue) * direction.value
            } else {
                currPos.South += Math.abs(yValue) * direction.value;
            }
            if (xValue > 0) {
                currPos.East += Math.abs(xValue) * direction.value
            } else {
                currPos.West += Math.abs(xValue) * direction.value;
            }
            break;
        default:
            waypoint = setWaypoint(direction.value,direction.action);
    }
});

const solution = Math.abs(currPos.North - currPos.South) + Math.abs(currPos.East - currPos.West)

console.log(solution);

function setWaypoint(moveBy, direction) {
    let newDir = "";
    let newWaypoint = {North:0,South:0,East:0,West:0};
    let xDir = "";
    let yDir = "";
    if (waypoint.North - waypoint.South > 0) {
        yDir = "North";
    } else {
        yDir = "South";
    }

    if (waypoint.East - waypoint.West > 0) {
        xDir = "East";
    } else {
        xDir = "West";
    }
    newDir = setDir(moveBy,direction,yDir);
    newWaypoint[newDir] = Math.abs(waypoint.North - waypoint.South); 
    newDir = setDir(moveBy,direction,xDir);
    newWaypoint[newDir] = Math.abs(waypoint.East - waypoint.West);
    return newWaypoint;
}

function setDir(moveBy, direction, currDir) {
    let degree = 0;
    switch(currDir)    {
        case 0:
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