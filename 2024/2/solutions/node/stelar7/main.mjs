import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);


const isSafe = (levels) => {
    let isIncreasing = false;
    for (let i = 0; i < levels.length - 1; i++) {
        if (i > 0) {
            if (isIncreasing && levels[i] > levels[i + 1]) {
                return false;
            } else if (!isIncreasing && levels[i] < levels[i + 1]) {
                return false;
            }
        } else {
            if (levels[i] < levels[i + 1]) {
                isIncreasing = true;
            }
        }

        const difference = Math.abs(levels[i] - levels[i + 1]);

        if (difference > 3 || difference < 1) {
            return false;
        }
    }
    return true;
};

let part1 = 0;
let part2 = 0;
lines.forEach((line) => {
    const levels = line.split(" ").map(Number);
    
    if (isSafe(levels)) {
        part1++;
        part2++;
    } else {
        for (let i = 0; i < levels.length; i++) {
            const options = levels.toSpliced(i, 1);
            if (isSafe(options)) {
                part2++;
                break;
            }
        }
    }
});

console.log(part1);
console.log(part2);
