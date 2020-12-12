const fs = require('fs');
const file  = fs.readFileSync('data.txt','utf8');
const dataArray = file.split("\r\n\r\n");

let totalQuestionsAns = 0;
dataArray.forEach(sample => {
    let answers = "";
    answers = sample.replace(/[\r\n]+/g,"");
    let uniqueAns = [];
    for (i = 0; i < answers.length; i++) {
        if (uniqueAns.indexOf(answers.charAt(i)) === -1) {
            uniqueAns.push(answers.charAt(i));
        }
    }
    totalQuestionsAns += uniqueAns.length;
})

console.log("Total questions answered: ", totalQuestionsAns)
