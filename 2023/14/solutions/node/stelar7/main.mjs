import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const transpose = (array) =>
  array[0].map((_, colIndex) => array.map((row) => row[colIndex]));

const rotateRight = (array) => transpose(array).map((row) => row.reverse());

const tilt = (array) => {
  const rotated = rotateRight(array);
  const tilted = rotated.map((row) => {
    for (let i = row.length - 1; i >= 0; i--) {
      if (row[i] == "O") {
        while (row[i + 1] == ".") {
          row[i + 1] = "O";
          row[i] = ".";
          i++;
        }
      }
    }
    return row;
  });

  return rotateRight(rotateRight(rotateRight(tilted)));
};

const calculateLoad = (array) => {
  let load = 0;
  for (let y = 0; y < array.length; y++) {
    const rowLoadMultiplier = array.length - y;
    const rockCount = array[y].filter((cell) => cell == "O").length;
    load += rockCount * rowLoadMultiplier;
  }

  return load;
};

const doCycle = (array) => {
  let transformed = array;
  for (let i = 0; i < 4; i++) {
    transformed = tilt(transformed);
    transformed = rotateRight(transformed);
  }

  return transformed;
};

const grid = lines.map((row) => row.split(""));
const tilted = tilt(grid);

console.log(calculateLoad(tilted));

let cache = {};
let newGrid = grid;
const cycleToCheck = 1_000_000_000;
for (let i = 1; i < cycleToCheck; i++) {
  newGrid = doCycle(newGrid);
  const key = newGrid.map((row) => row.join("")).join("");
  const entry = cache[key];
  if (!entry) {
    cache[key] = i;
  } else {
    const startOfLoop = entry;
    const loopLength = i - entry;
    const remainingCycles = cycleToCheck - startOfLoop;
    const cyclesToEnd = remainingCycles % loopLength;
    for (let j = 0; j < cyclesToEnd; j++) {
      newGrid = doCycle(newGrid);
    }
    console.log(calculateLoad(newGrid));
    break;
  }
}
