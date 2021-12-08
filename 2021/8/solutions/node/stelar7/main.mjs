import { S_IFREG } from "constants";
import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n");

function isSuperset(a, b) {
    return a.every(item => b.includes(item));
}

function isSubset(a, b) {
    return !isSuperset(b, a);
}

let outputCounts = Array(10).fill(0);
let outputSum = 0;
input.forEach(line => {
    if (!line.includes("|")) {
        return;
    }

    const [inputSegments, outputSegments] = line.split("|");
    outputSegments.split(" ").map(s => s.length).forEach(e => {
        outputCounts[e]++;
    });

    const segments = [...inputSegments.split(" ")];
    let digits = [];
    let fives = [];
    let sixes = [];

    for (let segment of segments) {
        switch (segment.length) {
            case 2:
                digits[1] = segment.split("").sort();
                break;
            case 3:
                digits[7] = segment.split("").sort();
                break;
            case 4:
                digits[4] = segment.split("").sort();
                break;
            case 5:
                fives.push(segment.split("").sort());
                break;
            case 6:
                sixes.push(segment.split("").sort());
                break;
            case 7:
                digits[8] = segment.split("").sort();
                break;
        }
    }

    
    for (let test of sixes) {
        if (isSubset(test, digits[1])) {
            digits[6] = test;
        } else if (isSubset(test, digits[4])) {
            digits[0] = test;
        } else {
            digits[9] = test;
        }
    }

    for (let test of fives) {
        if (isSuperset(test, digits[6])) {
            digits[5] = test;
        } else if (isSuperset(test, digits[9])) {
            digits[3] = test;
        } else {
            digits[2] = test;
        }
    }

    const output = outputSegments.split(" ").filter(e => e).map(a => {
        const test = a.split("").sort();
        return digits.findIndex(e => JSON.stringify(e) == JSON.stringify(test));
    });

    outputSum += +output.join("");
});

console.log(outputCounts[2] + outputCounts[3] + outputCounts[4] + outputCounts[7]);
console.log(outputSum);