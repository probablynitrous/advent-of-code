const fs= require('fs');6
const file = fs.readFileSync("data.txt", "utf8");
const dataArr = file.split("\r\n");

dataArr.push(0);
dataArr.sort((a, b) => a - b);
dataArr.push(parseInt(dataArr[dataArr.length-1]) + 3)

let memory = {};
const solution = findCombo(dataArr,memory);
console.log(solution);

function findCombo(array, memory) {
    const currArr = array.join();
    if (currArr in memory) {
        return memory[currArr];
    }
    let waysToGo = 1;
    for (let i = 1; i < array.length - 1; i ++) {
        let max = array[i+1];
        let min = array[i-1];
        if(max - min <= 3) {
            let newArr = array.slice(i+1);
            newArr.unshift(min);
            waysToGo += findCombo(newArr,memory);
        }
    }
    memory[currArr] = waysToGo;
    return waysToGo;
}