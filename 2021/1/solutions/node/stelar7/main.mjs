import { readFileSync } from "fs";

const STANDARD_IN = 0
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1)

let prev = null;
let count = 0;
for (const line of lines) {
  const current = parseInt(line);
  if (prev != null && current > prev) count++;
  prev = current;
}

console.log(`${count}`);

count = 0;
prev = null;
for (let i = 0; i<lines.length; i++) {
  const current = parseInt(lines[i]) + parseInt(lines[i + 1]) + parseInt(lines[i + 2]);

  if (prev != null && current > prev) {
    count++;
  }
  prev = current;
}

console.log(`${count}`);