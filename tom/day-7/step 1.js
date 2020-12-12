var fs = require('fs');
var file = fs.readFileSync('data.txt','utf8');
var dataArr = Array.from(file.split("\r\n"));

let bags = [];
dataArr.forEach(line => {
    let bagsObj = {};
    const regex = /^(.*) contain (.*)./;
    let result = line.match(regex);
    bagsObj.bag = result[1];
    let contentArr = result[2].split(", ");
    let bagContent = [];
    contentArr.forEach(content => {
        if (content !== "no other bags") {
            let bagContentObj = content.match(/(?<count>[0-9]) (?<containedBag>[a-z ]+)/)
            bagContent.push(bagContentObj.groups);
        }
    });
    bagsObj.content = bagContent;
    bags.push(bagsObj);
})
let canHoldGold = [];
bags.forEach(line => {
    let shinyGoldLine = line.content.filter(contents => contents.containedBag.indexOf("shiny gold bags") !== -1);
    if (shinyGoldLine.length !== 0) {
        if (canHoldGold.indexOf(line.bag) === -1) {
            canHoldGold.push(line.bag);
            findWhereUsed(line.bag);
        }
    }
})

console.log(canHoldGold);
console.log(canHoldGold.length);

function findWhereUsed(searchValue) {
    
    bags.forEach(bag => {
        bag.content.forEach(bagContent => {
            if (bagContent.containedBag === searchValue || bagContent.containedBag === searchValue.slice(0,-1)) {
                if (canHoldGold.indexOf(bag.bag) === -1) {
                    canHoldGold.push(bag.bag);
                    findWhereUsed(bag.bag)
                }
            }
        })
    });
}