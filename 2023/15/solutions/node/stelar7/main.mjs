import { readFileSync, writeFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const hashString = (str) => {
  let value = 0;
  for (let i = 0; i < str.length; i++) {
    value += str.charCodeAt(i);
    value *= 17;
    value %= 256;
  }
  return value;
};

let sum = 0;
const instructions = lines[0].split(",");
for (const intruction of instructions) {
  sum += hashString(intruction);
}

console.log(sum);

const boxes = {};
for (const intruction of instructions) {
  const hasDash = intruction.includes("-");
  const hasEquals = intruction.includes("=");

  if (hasDash) {
    const label = intruction.split("-")[0];
    const boxIndex = hashString(label);
    boxes[boxIndex] = boxes[boxIndex] || [];
    const labelIndex = boxes[boxIndex].findIndex((e) => e.label === label);

    if (labelIndex == -1) {
      continue;
    } else {
      boxes[boxIndex].splice(labelIndex, 1);
    }
  }

  if (hasEquals) {
    const label = intruction.split("=")[0];
    const focalLength = parseInt(intruction.split("=")[1]);
    const boxIndex = hashString(label);
    boxes[boxIndex] = boxes[boxIndex] || [];
    const labelIndex = boxes[boxIndex].findIndex((e) => e.label === label);

    if (labelIndex == -1) {
      boxes[boxIndex].push({ label, value: focalLength });
    } else {
      boxes[boxIndex].splice(labelIndex, 1, {
        label,
        value: focalLength,
      });
    }
  }
}

let totalFocusPower = 0;
for (let i = 0; i < 256; i++) {
  if (!boxes[i]) {
    continue;
  }

  for (let j = 0; j < boxes[i].length; j++) {
    const power = (1 + i) * (1 + j) * boxes[i][j].value;
    totalFocusPower += power;
  }
}

console.log(totalFocusPower);
