const fs= require('fs');
const file = fs.readFileSync("data.txt", "utf8");
const dataArr = file.split("\r\n");

let jolt1 = 1;
let jolt3 = 1;
dataArr.sort((a, b) => a - b);

for (i = 1; i < dataArr.length; i++) {
   const dif = dataArr[i] - dataArr[i-1];
   if (dif === 1 ) {
       jolt1 += 1;
   }
   if (dif === 3) {
       jolt3 += 1;
   }
}

console.log(jolt1 * jolt3);