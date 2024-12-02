import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const leftList = [];
const rightList = [];

lines.forEach((line) => {
    const [left, right] = line.split("   ").map(Number);
    leftList.push(left);
    rightList.push(right);
});

leftList.sort((a, b) => a - b);
rightList.sort((a, b) => a - b);

let part1 = 0;
for (let index = 0; index < leftList.length; index++) {
    const leftElement = leftList[index];
    const rightElement = rightList[index];

    const sum = Math.abs(leftElement - rightElement);
    part1 += sum;
}

console.log(part1);

const instanceCount = rightList.reduce((acc, rightElement) => {
    acc[rightElement] = (acc[rightElement] || 0) + 1;
    return acc;
}, {});

let part2 = 0;
for (let index = 0; index < leftList.length; index++) {
    const leftElement = leftList[index];

    const rightCount = instanceCount[leftElement] ?? 0;

    const similarityScore = leftElement * rightCount;
    part2 += similarityScore;
}

console.log(part2);