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

const score = invalidChars.map(c => {
        if (c == ")") return 3;
        if (c == "]") return 57;
        if (c == "}") return 1197;
        if (c == ">") return 25137;
    })
    .reduce((o, n) => o + n, 0);

console.log(score);

const autocompleteScores = incompleteLines.map(n => {
        return n.reverse()
            .map(c => invert(c))
            .map(c => {
                if (c == ")") return 1;
                if (c == "]") return 2;
                if (c == "}") return 3;
                if (c == ">") return 4;
            })
            .reduce((a, b) => (a * 5) + b, 0);
    })
    .sort((a, b) => b - a);

const centerpoint = (autocompleteScores.length / 2) - 1;
console.log(autocompleteScores[centerpoint]);