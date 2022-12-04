import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

let score = 0;
for (const line of lines) {
    const first = line.slice(0, line.length / 2);
    const second = line.slice(line.length / 2, line.length);

    const firstList = new Set(first.split(""));
    firstList.forEach(char => {
        if (second.includes(char)) {
            if (char == char.toLowerCase()) {
                score += (char.charCodeAt(0) - "a".charCodeAt(0) + 1);
            } else {
                score += (char.charCodeAt(0) - "A".charCodeAt(0) + 27);
            }
        }
    });
}

console.log(`${score}`);

score = 0;
for (let i = 0; i < lines.length; i += 3) {
    const line1 = lines[i + 0];
    const line2 = lines[i + 1];
    const line3 = lines[i + 2];

    const firstList = new Set(line1.split(""));
    firstList.forEach(char => {
        if (line2.includes(char) && line3.includes(char)) {
            if (char == char.toLowerCase()) {
                score += (char.charCodeAt(0) - "a".charCodeAt(0) + 1);
            } else {
                score += (char.charCodeAt(0) - "A".charCodeAt(0) + 27);
            }
        }
    });
}

console.log(`${score}`);