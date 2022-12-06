import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const line = lines[0];
for (let i = 0; i < line.length - 4; i++) {
    let unique = new Set();
    for (let j = i; j < i+4; j++) {
        unique.add(line[j]);
    }
    if (unique.size == 4) {
        console.log(`${i + 4}`);
        break;
    }
}

for (let i = 0; i < line.length - 4; i++) {
    let unique2 = new Set();
    for (let j = i; j < i+14; j++) {
        unique2.add(line[j]);
    }
    if (unique2.size == 14) {
        console.log(`${i + 14}`);
        break;
    }
}