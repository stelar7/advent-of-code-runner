import { readFileSync } from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split(",").map(n => +n);

const max = Math.max(...input);
const min = Math.min(...input);

let lowest = Number.MAX_VALUE;
for (let i = min; i <= max; i++) {
    const test = input.reduce((o, n) => o + Math.abs(n - i), 0);
    lowest = Math.min(lowest, test);
}
console.log(lowest);

lowest = Number.MAX_VALUE;
for (let i = min; i <= max; i++) {
    const test = input.reduce((o, n) => {
        const abs = Math.abs(n - i);
        return o + (abs * (abs + 1)) / 2;
    }, 0);
    
    lowest = Math.min(lowest, test);
}

console.log(lowest);
