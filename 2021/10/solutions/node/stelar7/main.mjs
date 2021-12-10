import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n");


function isClosing(c) {
    return c == ")" || c == "}" || c == ">" || c == "]";
}

function isMatching(expected, actual) {
    if (expected == "(") return ")" == actual;
    if (expected == "[") return "]" == actual;
    if (expected == "{") return "}" == actual;
    if (expected == "<") return ">" == actual;
}

function invert(c) {
    if (c == "(") return ")";
    if (c == "[") return "]";
    if (c == "{") return "}";
    if (c == "<") return ">";
}

const incompleteLines = [];
const invalidChars = [];
input.forEach(line => {
    const openList = [];

    for (let char of line) {
        const lastInList = openList[openList.length - 1];

        if (isClosing(char)) {
            if (isMatching(lastInList, char)) {
                openList.pop();
            } else {
                invalidChars.push(char);
                return;
            }
        } else {
            openList.push(char);
        }
    }
    incompleteLines.push(openList);
});

const score = invalidChars.reduce((o, n) => {
    let score = 0;
    if (n == ")") score = 3;
    if (n == "]") score = 57;
    if (n == "}") score = 1197;
    if (n == ">") score = 25137;
    return o + score;
}, 0);

console.log(score);

const autocompleteScores = incompleteLines.map(n => {
        return n.reverse()
            .map(c => invert(c))
            .reduce((a, b) => {
                let score = 0;
                if (b == ")") score = 1;
                if (b == "]") score = 2;
                if (b == "}") score = 3;
                if (b == ">") score = 4;
                return (a * 5) + score;
            }, 0);
    })
    .sort((a, b) => b - a);

console.log(autocompleteScores[autocompleteScores.length / 2 -1]);