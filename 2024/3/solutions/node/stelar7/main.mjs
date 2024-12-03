import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const mulRegex = /^(mul\(\d{1,3},\d{1,3}\)$)/mg;
let part1 = 0;
let part2 = 0;
let mulEnabled = true;

const line = lines.join("");

const nextIs = (line, i, str) => {
    for (let j = 0; j < str.length && (i + j) < line.length; j++) {
        if (line[i + j] != str[j]) {
            return false;
        }
    }
    return true;
};

for (let i = 0; i < line.length; i++) {
    if (nextIs(line, i, "do()")) {
        i+="do()".length;
        mulEnabled = true;
    }

    if (nextIs(line, i, "don't()")) {
        i+="don't()".length;
        mulEnabled = false;
    }

    if (nextIs(line, i, "mul(")) {
        i+="mul(".length;

        const endIndexOfMul = line.indexOf(")", i);
        const mulString = line.substring(i - "mul(".length, endIndexOfMul + 1);
        const mulNumbers = line.substring(i, endIndexOfMul);
        
        if (mulString.match(mulRegex)) {
            const [a, b] = mulNumbers.split(",").map(Number);
            part1 += a * b;

            if (mulEnabled) {
                part2 += a * b;
            }
        }
    }
}

console.log(part1);
console.log(part2);