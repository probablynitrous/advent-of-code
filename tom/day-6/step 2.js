const fs = require('fs');
const file  = fs.readFileSync('data.txt','utf8');
const dataArray = file.split("\r\n\r\n");

let totalQuestionsAns = 0;
dataArray.forEach(sample => {
    let answers = [];
    answers = sample.split("\r\n");
    let commonAns = findCommonValues(answers);
    totalQuestionsAns += commonAns.length;
})

console.log("Total questions answered by all: ", totalQuestionsAns)

function findCommonValues(answers) {
    let result = [];
    for (i = 0; i < answers[0].length; i++) {
        let searchChar = answers[0].charAt(i);
        let isMatch = true;
        for (j = 1; j < answers.length; j++) {
            if (answers[j].indexOf(searchChar) === -1) {
                isMatch = false;
            }
        }
        if (isMatch) {
            result.push(searchChar);
        }
    }
    return(result);
}