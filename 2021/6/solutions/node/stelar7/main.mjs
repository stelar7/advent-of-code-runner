import { readFileSync } from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split(",").map(n => +n);

let cycles = [0, 0, 0, 0, 0, 0, 0, 0, 0];
input.forEach(n => cycles[n]++);


for (let i = 0; i < 80; i++) {
    const [countNew, ...rest] = cycles;
    cycles = [...rest, countNew];
    cycles[6] += countNew;
}

console.log(cycles.reduce((o, n) => o + n, 0));

for (let i = 0; i < (256 - 80); i++) {
    const [countNew, ...rest] = cycles;
    cycles = [...rest, countNew];
    cycles[6] += countNew;
}

console.log(cycles.reduce((o, n) => o + n, 0));

