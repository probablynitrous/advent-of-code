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
let heldByGold = []
let bagsHeld = 0;
findWhereUsed("shiny gold bags",1)
console.log(bagsHeld);
console.log(heldByGold);

function findWhereUsed(searchValue, prevBagsHeld) {
    let filteredBags = bags.filter(bags => bags.bag.indexOf(searchValue) !== -1);
    if (filteredBags.length !== 0) {
        filteredBags[0].content.forEach(bagContent => {
            heldByGold.push(bagContent.containedBag);
            bagsInc = parseInt(bagContent.count) * parseInt(prevBagsHeld)
            bagsHeld += bagsInc;
            findWhereUsed(bagContent.containedBag, bagsInc); 
        });
    }
}