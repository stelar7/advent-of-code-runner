import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const parseLine = (line) => {
  const cards = line.split(":")[1];
  const [winners, mine] = cards.split("|");
  return [
    winners
      .split(" ")
      .map((s) => s.trim())
      .filter((a) => a)
      .map((s) => parseInt(s))
      .sort(),
    mine
      .split(" ")
      .map((s) => s.trim())
      .filter((a) => a)
      .map((s) => parseInt(s))
      .sort(),
  ];
};

let sum = 0;
const copies = lines.map((line) => ({
  line,
  copyCount: 1,
}));

for (let i = 0; i < copies.length; i++) {
  const { line, copyCount } = copies[i];
  const [winnersCards, mineCards] = parseLine(line);

  let count = 0;
  for (let winningCard of winnersCards) {
    if (mineCards.includes(winningCard)) {
      count++;
    }
  }

  for (let j = 1; j <= count; j++) {
    copies[i + j].copyCount += copyCount;
  }

  let lineScore = Math.pow(2, count - 1);
  if (lineScore < 1) {
    lineScore = 0;
  }

  sum += lineScore;
}

console.log(sum);
console.log(copies.reduce((acc, { copyCount }) => acc + copyCount, 0));
