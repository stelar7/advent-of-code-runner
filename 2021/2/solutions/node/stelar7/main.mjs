import { readFileSync } from "fs";

const STANDARD_IN = 0
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1)

const directions = {
  "up": -1,
  "down": 1,
  "forward": 1,
  "backward": -1,
};

let x = 0;
let y = 0;
let aim = 0;
let depth = 0;
for (const line of lines) {
  const [direction, amount] = line.split(" ");
  if (direction == "up" || direction == "down") {
    y += amount * directions[direction];
    aim += amount * directions[direction];
  } else {
    x += amount * directions[direction];
    depth += aim * amount;
  }
}

console.log(`${x * y}`);
console.log(`${x * depth}`);