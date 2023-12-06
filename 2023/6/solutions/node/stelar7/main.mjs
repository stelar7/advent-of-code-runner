import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const times = lines[0]
  .split(" ")
  .filter((a) => a)
  .slice(1)
  .map((x) => parseInt(x));
const distances = lines[1]
  .split(" ")
  .filter((a) => a)
  .slice(1)
  .map((x) => parseInt(x));

function solveQuadratic(time, distance) {
  const bias = 0.001;

  const one = Math.ceil((time - Math.sqrt(Math.pow(time, 2) - 4 * distance)) / 2 + bias);
  const two = Math.floor((time + Math.sqrt(Math.pow(time, 2) - 4 * distance)) / 2 - bias);

  return two - one + 1;
}

const allWins = times.map((time, i) => solveQuadratic(time, distances[i]));
console.log(allWins.reduce((a, b) => a * b, 1));

const finalRaceTime = parseInt(times.join(""));
const finalDistance = parseInt(distances.join(""));
console.log(solveQuadratic(finalRaceTime, finalDistance));
